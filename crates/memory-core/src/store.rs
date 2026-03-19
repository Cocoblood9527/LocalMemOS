use std::cmp::Ordering;
use std::collections::HashSet;
use std::sync::atomic::{AtomicU64, Ordering as AtomicOrdering};
use std::time::{SystemTime, UNIX_EPOCH};

use chrono::{DateTime, Utc};
use rusqlite::OptionalExtension;

use crate::error::MemoryError;
use crate::fts::{build_fts_query, normalize_query_sequence, normalize_query_tokens, upsert_fts_row};
use crate::model::{FactRecord, FactVersionRecord, RecallResponse};
use crate::request::{ForgetRequest, ListRequest, RecallRequest, UpsertFactRequest};

static ID_COUNTER: AtomicU64 = AtomicU64::new(1);

pub struct MemoryStore {
    conn: rusqlite::Connection,
}

impl MemoryStore {
    pub fn open(path: &str) -> rusqlite::Result<Self> {
        let conn = rusqlite::Connection::open(path)?;
        conn.execute_batch(crate::schema::ALL_SCHEMA)?;
        Ok(Self { conn })
    }

    pub fn table_count(&self) -> rusqlite::Result<i64> {
        self.conn.query_row(
            "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name NOT LIKE 'sqlite_%'",
            [],
            |row| row.get(0),
        )
    }

    pub fn upsert_fact(&mut self, req: UpsertFactRequest) -> Result<FactRecord, MemoryError> {
        req.validate()?;
        let tx = self.conn.transaction()?;
        let now = Utc::now();
        let now_text = now.to_rfc3339();

        if let Some(current) = find_current_fact(&tx, &req)? {
            if current.value_json == req.value {
                tx.execute(
                    "UPDATE facts SET updated_at = ? WHERE id = ?",
                    rusqlite::params![now_text, current.id],
                )?;
                append_evidence(&tx, &current.id, None, &req, &now_text)?;
                tx.commit()?;
                let mut refreshed = current;
                refreshed.updated_at = now;
                return Ok(refreshed);
            }

            close_current_fact(&tx, &current.id, &now_text)?;
            close_current_version(&tx, &req, &now_text)?;
        }

        let inserted = insert_new_fact(&tx, &req, now)?;
        let version_id = insert_fact_version(&tx, &inserted, &req, &now_text)?;
        append_evidence(&tx, &inserted.id, Some(&version_id), &req, &now_text)?;
        upsert_fts_row(
            &tx,
            &inserted.id,
            &inserted.namespace,
            &inserted.scope_id,
            &inserted.entity,
            &inserted.attribute,
            inserted.value_text.as_deref(),
            req.evidence_summary.as_deref(),
        )?;
        tx.commit()?;
        Ok(inserted)
    }

    pub fn get_fact(
        &self,
        namespace: &str,
        scope_id: &str,
        entity: &str,
        attribute: &str,
    ) -> Result<Option<FactRecord>, MemoryError> {
        let raw = self
            .conn
            .query_row(
                "SELECT id, namespace, scope_id, entity, attribute, value_json, value_text, confidence, valid_from, valid_to, updated_at
                 FROM facts
                 WHERE namespace = ? AND scope_id = ? AND entity = ? AND attribute = ? AND valid_to IS NULL
                 LIMIT 1",
                rusqlite::params![namespace, scope_id, entity, attribute],
                |row| {
                    Ok(RawFact {
                        id: row.get(0)?,
                        namespace: row.get(1)?,
                        scope_id: row.get(2)?,
                        entity: row.get(3)?,
                        attribute: row.get(4)?,
                        value_json: row.get(5)?,
                        value_text: row.get(6)?,
                        confidence: row.get(7)?,
                        valid_from: row.get(8)?,
                        valid_to: row.get(9)?,
                        updated_at: row.get(10)?,
                    })
                },
            )
            .optional()?;

        match raw {
            Some(raw_fact) => Ok(Some(raw_to_fact(raw_fact)?)),
            None => Ok(None),
        }
    }

    pub fn history(
        &self,
        namespace: &str,
        scope_id: &str,
        entity: &str,
        attribute: &str,
    ) -> Result<Vec<FactVersionRecord>, MemoryError> {
        let mut stmt = self.conn.prepare(
            "SELECT id, fact_id, namespace, scope_id, entity, attribute, value_json, value_text, confidence, valid_from, valid_to, created_at
             FROM fact_versions
             WHERE namespace = ? AND scope_id = ? AND entity = ? AND attribute = ?
             ORDER BY created_at ASC",
        )?;
        let raw_rows = stmt.query_map(
            rusqlite::params![namespace, scope_id, entity, attribute],
            |row| {
                Ok(RawFactVersion {
                    id: row.get(0)?,
                    fact_id: row.get(1)?,
                    namespace: row.get(2)?,
                    scope_id: row.get(3)?,
                    entity: row.get(4)?,
                    attribute: row.get(5)?,
                    value_json: row.get(6)?,
                    value_text: row.get(7)?,
                    confidence: row.get(8)?,
                    valid_from: row.get(9)?,
                    valid_to: row.get(10)?,
                    created_at: row.get(11)?,
                })
            },
        )?;

        let mut out = Vec::new();
        for raw in raw_rows {
            out.push(raw_to_fact_version(raw?)?);
        }
        Ok(out)
    }

    pub fn history_by_fact_id(
        &self,
        fact_id: &str,
    ) -> Result<Option<Vec<FactVersionRecord>>, MemoryError> {
        let fact_key = self
            .conn
            .query_row(
                "SELECT namespace, scope_id, entity, attribute
                 FROM facts
                 WHERE id = ?
                 LIMIT 1",
                rusqlite::params![fact_id],
                |row| {
                    Ok((
                        row.get::<_, String>(0)?,
                        row.get::<_, String>(1)?,
                        row.get::<_, String>(2)?,
                        row.get::<_, String>(3)?,
                    ))
                },
            )
            .optional()?;

        match fact_key {
            Some((namespace, scope_id, entity, attribute)) => Ok(Some(
                self.history(&namespace, &scope_id, &entity, &attribute)?,
            )),
            None => Ok(None),
        }
    }

    pub fn recall(&self, req: RecallRequest) -> Result<RecallResponse, MemoryError> {
        let effective_at = req.as_of.unwrap_or_else(Utc::now);

        if let Some(text_query) = req.text_query.as_deref() {
            return self.recall_with_fts(&req.namespace, &req.scope_id, text_query, effective_at);
        }

        let facts = self.query_current_facts(
            &req.namespace,
            &req.scope_id,
            req.entity.as_deref(),
            req.attribute.as_deref(),
            effective_at,
        )?;
        Ok(RecallResponse { facts })
    }

    fn recall_with_fts(
        &self,
        namespace: &str,
        scope_id: &str,
        text_query: &str,
        effective_at: DateTime<Utc>,
    ) -> Result<RecallResponse, MemoryError> {
        let query_tokens = normalize_query_tokens(text_query);
        let query_sequence = normalize_query_sequence(text_query);
        let query_bigrams = bigram_set(&query_sequence);
        let long_query = query_sequence.len() >= 6;
        let fts_query = match build_fts_query(text_query) {
            Some(q) => q,
            None => return Ok(RecallResponse { facts: Vec::new() }),
        };
        let effective_at_text = effective_at.to_rfc3339();
        let mut stmt = self.conn.prepare(
            "SELECT DISTINCT f.id, f.namespace, f.scope_id, f.entity, f.attribute, f.value_json, f.value_text, f.confidence, f.valid_from, f.valid_to, f.updated_at,
                             facts_fts.entity, facts_fts.attribute, facts_fts.value_text, facts_fts.evidence_summary,
                             bm25(facts_fts, 0.0, 0.0, 1.4, 1.8, 2.4, 1.2) AS bm25_rank
             FROM facts f
             INNER JOIN facts_fts ON facts_fts.fact_id = f.id
             WHERE f.namespace = ?1 AND f.scope_id = ?2
               AND julianday(f.valid_from) <= julianday(?3)
               AND (f.valid_to IS NULL OR julianday(f.valid_to) > julianday(?3))
               AND facts_fts MATCH ?4
             ORDER BY bm25_rank ASC, f.updated_at DESC",
        )?;
        let raw_rows = stmt.query_map(
            rusqlite::params![namespace, scope_id, effective_at_text, fts_query],
            |row| {
                Ok(RankedFactRow {
                    raw: RawFact {
                        id: row.get(0)?,
                        namespace: row.get(1)?,
                        scope_id: row.get(2)?,
                        entity: row.get(3)?,
                        attribute: row.get(4)?,
                        value_json: row.get(5)?,
                        value_text: row.get(6)?,
                        confidence: row.get(7)?,
                        valid_from: row.get(8)?,
                        valid_to: row.get(9)?,
                        updated_at: row.get(10)?,
                    },
                    fts_entity: row.get(11)?,
                    fts_attribute: row.get(12)?,
                    fts_value_text: row.get(13)?,
                    fts_evidence_summary: row.get(14)?,
                    bm25_rank: row.get(15)?,
                })
            },
        )?;

        let mut ranked_facts = Vec::new();
        for raw in raw_rows {
            let row = raw?;
            let candidate_sequence = candidate_sequence(
                &query_tokens,
                &row.fts_entity,
                &row.fts_attribute,
                row.fts_value_text.as_deref(),
                row.fts_evidence_summary.as_deref(),
            );
            let token_coverage = token_coverage_ratio(&query_tokens, &candidate_sequence);
            let candidate_bigrams = bigram_set(&candidate_sequence);
            let bigram_coverage = bigram_coverage_ratio(&query_bigrams, &candidate_bigrams);
            let longest_run = longest_contiguous_run(&query_sequence, &candidate_sequence);
            let continuity_score = if query_sequence.is_empty() {
                0.0
            } else {
                longest_run as f64 / query_sequence.len() as f64
            };
            let phrase_bonus = if longest_run >= 5 { 1.0 } else { 0.0 };
            let bigram_weight = if long_query { 0.5 } else { 2.2 };
            let continuity_weight = if long_query { 0.4 } else { 1.2 };
            let phrase_weight = if long_query { 0.1 } else { 0.6 };
            let rerank_score = (token_coverage * if long_query { 1.2 } else { 3.0 })
                + (bigram_coverage * bigram_weight)
                + (continuity_score * continuity_weight)
                + (phrase_bonus * phrase_weight);
            ranked_facts.push((raw_to_fact(row.raw)?, row.bm25_rank, rerank_score));
        }

        if long_query {
            ranked_facts.sort_by(|left, right| {
                left.1
                    .partial_cmp(&right.1)
                    .unwrap_or(Ordering::Equal)
                    .then_with(|| right.2.partial_cmp(&left.2).unwrap_or(Ordering::Equal))
                    .then_with(|| right.0.updated_at.cmp(&left.0.updated_at))
            });
        } else {
            ranked_facts.sort_by(|left, right| {
                right
                    .2
                    .partial_cmp(&left.2)
                    .unwrap_or(Ordering::Equal)
                    .then_with(|| left.1.partial_cmp(&right.1).unwrap_or(Ordering::Equal))
                    .then_with(|| right.0.updated_at.cmp(&left.0.updated_at))
            });
        }

        let facts = ranked_facts
            .into_iter()
            .map(|(fact, _, _)| fact)
            .collect::<Vec<FactRecord>>();
        Ok(RecallResponse { facts })
    }

    pub fn list(&self, req: ListRequest) -> Result<RecallResponse, MemoryError> {
        let facts = self.query_current_facts(
            &req.namespace,
            &req.scope_id,
            req.entity.as_deref(),
            req.attribute.as_deref(),
            Utc::now(),
        )?;
        Ok(RecallResponse { facts })
    }

    pub fn forget(&mut self, req: ForgetRequest) -> Result<(), MemoryError> {
        let now = Utc::now().to_rfc3339();
        let namespace = req.namespace;
        let scope_id = req.scope_id;
        let entity = req.entity;
        let attribute = req.attribute;
        self.conn.execute(
            "UPDATE facts
             SET valid_to = ?, status = 'retired', updated_at = ?
             WHERE namespace = ? AND scope_id = ? AND entity = ? AND attribute = ? AND valid_to IS NULL",
            rusqlite::params![
                &now,
                &now,
                &namespace,
                &scope_id,
                &entity,
                &attribute
            ],
        )?;
        self.conn.execute(
            "UPDATE fact_versions
             SET valid_to = ?
             WHERE namespace = ? AND scope_id = ? AND entity = ? AND attribute = ? AND valid_to IS NULL",
            rusqlite::params![
                &now,
                &namespace,
                &scope_id,
                &entity,
                &attribute
            ],
        )?;
        Ok(())
    }

    fn query_current_facts(
        &self,
        namespace: &str,
        scope_id: &str,
        entity: Option<&str>,
        attribute: Option<&str>,
        effective_at: DateTime<Utc>,
    ) -> Result<Vec<FactRecord>, MemoryError> {
        let effective_at_text = effective_at.to_rfc3339();
        let mut stmt = self.conn.prepare(
            "SELECT id, namespace, scope_id, entity, attribute, value_json, value_text, confidence, valid_from, valid_to, updated_at
             FROM facts
             WHERE namespace = ?1 AND scope_id = ?2
               AND julianday(valid_from) <= julianday(?3)
               AND (valid_to IS NULL OR julianday(valid_to) > julianday(?3))
               AND (?4 IS NULL OR entity = ?4)
               AND (?5 IS NULL OR attribute = ?5)
             ORDER BY updated_at DESC",
        )?;
        let raw_rows = stmt.query_map(
            rusqlite::params![namespace, scope_id, effective_at_text, entity, attribute],
            |row| {
                Ok(RawFact {
                    id: row.get(0)?,
                    namespace: row.get(1)?,
                    scope_id: row.get(2)?,
                    entity: row.get(3)?,
                    attribute: row.get(4)?,
                    value_json: row.get(5)?,
                    value_text: row.get(6)?,
                    confidence: row.get(7)?,
                    valid_from: row.get(8)?,
                    valid_to: row.get(9)?,
                    updated_at: row.get(10)?,
                })
            },
        )?;

        let mut out = Vec::new();
        for raw in raw_rows {
            out.push(raw_to_fact(raw?)?);
        }
        Ok(out)
    }
}

fn find_current_fact(
    tx: &rusqlite::Transaction<'_>,
    req: &UpsertFactRequest,
) -> Result<Option<FactRecord>, MemoryError> {
    let raw = tx
        .query_row(
            "SELECT id, namespace, scope_id, entity, attribute, value_json, value_text, confidence, valid_from, valid_to, updated_at
             FROM facts
             WHERE namespace = ? AND scope_id = ? AND entity = ? AND attribute = ? AND valid_to IS NULL
             LIMIT 1",
            rusqlite::params![req.namespace, req.scope_id, req.entity, req.attribute],
            |row| {
                Ok(RawFact {
                    id: row.get(0)?,
                    namespace: row.get(1)?,
                    scope_id: row.get(2)?,
                    entity: row.get(3)?,
                    attribute: row.get(4)?,
                    value_json: row.get(5)?,
                    value_text: row.get(6)?,
                    confidence: row.get(7)?,
                    valid_from: row.get(8)?,
                    valid_to: row.get(9)?,
                    updated_at: row.get(10)?,
                })
            },
        )
        .optional()?;

    match raw {
        Some(raw_fact) => Ok(Some(raw_to_fact(raw_fact)?)),
        None => Ok(None),
    }
}

fn close_current_fact(
    tx: &rusqlite::Transaction<'_>,
    fact_id: &str,
    now_text: &str,
) -> Result<(), MemoryError> {
    tx.execute(
        "UPDATE facts
         SET valid_to = ?, status = 'retired', updated_at = ?
         WHERE id = ?",
        rusqlite::params![now_text, now_text, fact_id],
    )?;
    Ok(())
}

fn close_current_version(
    tx: &rusqlite::Transaction<'_>,
    req: &UpsertFactRequest,
    now_text: &str,
) -> Result<(), MemoryError> {
    tx.execute(
        "UPDATE fact_versions
         SET valid_to = ?
         WHERE namespace = ? AND scope_id = ? AND entity = ? AND attribute = ? AND valid_to IS NULL",
        rusqlite::params![now_text, req.namespace, req.scope_id, req.entity, req.attribute],
    )?;
    Ok(())
}

fn insert_new_fact(
    tx: &rusqlite::Transaction<'_>,
    req: &UpsertFactRequest,
    now: DateTime<Utc>,
) -> Result<FactRecord, MemoryError> {
    let id = generate_id("fact");
    let value_json = serde_json::to_string(&req.value)?;
    let value_text = value_as_text(&req.value);
    let valid_from = req.valid_from.unwrap_or(now);
    let valid_from_text = valid_from.to_rfc3339();
    let now_text = now.to_rfc3339();

    tx.execute(
        "INSERT INTO facts (
            id, namespace, scope_id, entity, attribute,
            value_json, value_text, confidence, status,
            valid_from, valid_to, created_at, updated_at
         ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, NULL, ?, ?)",
        rusqlite::params![
            id,
            req.namespace,
            req.scope_id,
            req.entity,
            req.attribute,
            value_json,
            value_text,
            req.confidence,
            "active",
            valid_from_text,
            now_text,
            now_text
        ],
    )?;

    Ok(FactRecord {
        id,
        namespace: req.namespace.clone(),
        scope_id: req.scope_id.clone(),
        entity: req.entity.clone(),
        attribute: req.attribute.clone(),
        value_json: req.value.clone(),
        value_text: value_as_text(&req.value),
        confidence: req.confidence,
        valid_from,
        valid_to: None,
        updated_at: now,
    })
}

fn insert_fact_version(
    tx: &rusqlite::Transaction<'_>,
    fact: &FactRecord,
    req: &UpsertFactRequest,
    now_text: &str,
) -> Result<String, MemoryError> {
    let id = generate_id("version");
    tx.execute(
        "INSERT INTO fact_versions (
            id, fact_id, namespace, scope_id, entity, attribute,
            value_json, value_text, confidence, valid_from, valid_to, created_at
         ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, NULL, ?)",
        rusqlite::params![
            id,
            fact.id,
            fact.namespace,
            fact.scope_id,
            fact.entity,
            fact.attribute,
            serde_json::to_string(&fact.value_json)?,
            fact.value_text,
            req.confidence,
            fact.valid_from.to_rfc3339(),
            now_text
        ],
    )?;
    Ok(id)
}

fn append_evidence(
    tx: &rusqlite::Transaction<'_>,
    fact_id: &str,
    fact_version_id: Option<&str>,
    req: &UpsertFactRequest,
    now_text: &str,
) -> Result<(), MemoryError> {
    tx.execute(
        "INSERT INTO evidence (
            id, fact_id, fact_version_id, source_kind, source_ref, summary, created_at
         ) VALUES (?, ?, ?, ?, ?, ?, ?)",
        rusqlite::params![
            generate_id("evidence"),
            fact_id,
            fact_version_id,
            req.source_kind,
            req.source_ref,
            req.evidence_summary,
            now_text
        ],
    )?;
    Ok(())
}

fn value_as_text(value: &serde_json::Value) -> Option<String> {
    match value {
        serde_json::Value::String(s) => Some(s.clone()),
        serde_json::Value::Null => None,
        _ => Some(value.to_string()),
    }
}

fn candidate_sequence(
    query_tokens: &[String],
    entity: &str,
    attribute: &str,
    value_text: Option<&str>,
    evidence_summary: Option<&str>,
) -> Vec<String> {
    if query_tokens.is_empty() {
        return Vec::new();
    }

    let mut candidate_tokens = Vec::new();
    candidate_tokens.extend(normalize_query_sequence(entity));
    candidate_tokens.extend(normalize_query_sequence(attribute));
    if let Some(text) = value_text {
        candidate_tokens.extend(normalize_query_sequence(text));
    }
    if let Some(text) = evidence_summary {
        candidate_tokens.extend(normalize_query_sequence(text));
    }
    candidate_tokens
}

fn token_coverage_ratio(query_tokens: &[String], candidate_sequence: &[String]) -> f64 {
    if query_tokens.is_empty() {
        return 0.0;
    }
    let candidate_set = candidate_sequence
        .iter()
        .cloned()
        .collect::<HashSet<String>>();
    let overlap = query_tokens
        .iter()
        .filter(|token| candidate_set.contains(*token))
        .count();
    overlap as f64 / query_tokens.len() as f64
}

fn bigram_set(tokens: &[String]) -> HashSet<(String, String)> {
    let mut out = HashSet::new();
    for window in tokens.windows(2) {
        out.insert((window[0].clone(), window[1].clone()));
    }
    out
}

fn bigram_coverage_ratio(
    query_bigrams: &HashSet<(String, String)>,
    candidate_bigrams: &HashSet<(String, String)>,
) -> f64 {
    if query_bigrams.is_empty() {
        return 0.0;
    }
    let overlap = query_bigrams
        .iter()
        .filter(|bigram| candidate_bigrams.contains(*bigram))
        .count();
    overlap as f64 / query_bigrams.len() as f64
}

fn longest_contiguous_run(query_sequence: &[String], candidate_sequence: &[String]) -> usize {
    if query_sequence.is_empty() || candidate_sequence.is_empty() {
        return 0;
    }

    let mut longest = 0;
    for q_start in 0..query_sequence.len() {
        for c_start in 0..candidate_sequence.len() {
            let mut run = 0;
            while q_start + run < query_sequence.len()
                && c_start + run < candidate_sequence.len()
                && query_sequence[q_start + run] == candidate_sequence[c_start + run]
            {
                run += 1;
            }
            if run > longest {
                longest = run;
            }
        }
    }
    longest
}

fn raw_to_fact(raw: RawFact) -> Result<FactRecord, MemoryError> {
    Ok(FactRecord {
        id: raw.id,
        namespace: raw.namespace,
        scope_id: raw.scope_id,
        entity: raw.entity,
        attribute: raw.attribute,
        value_json: serde_json::from_str(&raw.value_json)?,
        value_text: raw.value_text,
        confidence: raw.confidence,
        valid_from: parse_datetime(raw.valid_from)?,
        valid_to: parse_optional_datetime(raw.valid_to)?,
        updated_at: parse_datetime(raw.updated_at)?,
    })
}

fn raw_to_fact_version(raw: RawFactVersion) -> Result<FactVersionRecord, MemoryError> {
    Ok(FactVersionRecord {
        id: raw.id,
        fact_id: raw.fact_id,
        namespace: raw.namespace,
        scope_id: raw.scope_id,
        entity: raw.entity,
        attribute: raw.attribute,
        value_json: serde_json::from_str(&raw.value_json)?,
        value_text: raw.value_text,
        confidence: raw.confidence,
        valid_from: parse_datetime(raw.valid_from)?,
        valid_to: parse_optional_datetime(raw.valid_to)?,
        created_at: parse_datetime(raw.created_at)?,
    })
}

fn parse_datetime(value: String) -> Result<DateTime<Utc>, MemoryError> {
    Ok(DateTime::parse_from_rfc3339(&value)?.with_timezone(&Utc))
}

fn parse_optional_datetime(value: Option<String>) -> Result<Option<DateTime<Utc>>, MemoryError> {
    match value {
        Some(v) => Ok(Some(parse_datetime(v)?)),
        None => Ok(None),
    }
}

fn generate_id(prefix: &str) -> String {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    let seq = ID_COUNTER.fetch_add(1, AtomicOrdering::Relaxed);
    format!("{prefix}-{nanos}-{seq}")
}

struct RawFact {
    id: String,
    namespace: String,
    scope_id: String,
    entity: String,
    attribute: String,
    value_json: String,
    value_text: Option<String>,
    confidence: Option<f32>,
    valid_from: String,
    valid_to: Option<String>,
    updated_at: String,
}

struct RawFactVersion {
    id: String,
    fact_id: String,
    namespace: String,
    scope_id: String,
    entity: String,
    attribute: String,
    value_json: String,
    value_text: Option<String>,
    confidence: Option<f32>,
    valid_from: String,
    valid_to: Option<String>,
    created_at: String,
}

struct RankedFactRow {
    raw: RawFact,
    fts_entity: String,
    fts_attribute: String,
    fts_value_text: Option<String>,
    fts_evidence_summary: Option<String>,
    bm25_rank: f64,
}
