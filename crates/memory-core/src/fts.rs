pub fn upsert_fts_row(
    tx: &rusqlite::Transaction<'_>,
    fact_id: &str,
    namespace: &str,
    scope_id: &str,
    entity: &str,
    attribute: &str,
    value_text: Option<&str>,
    evidence_summary: Option<&str>,
) -> rusqlite::Result<()> {
    tx.execute(
        "INSERT INTO facts_fts (fact_id, namespace, scope_id, entity, attribute, value_text, evidence_summary)
         VALUES (?, ?, ?, ?, ?, ?, ?)",
        rusqlite::params![
            fact_id,
            namespace,
            scope_id,
            entity,
            attribute,
            value_text,
            evidence_summary
        ],
    )?;
    Ok(())
}
