use std::collections::HashSet;

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

const STOPWORDS: &[&str] = &[
    "a",
    "an",
    "and",
    "are",
    "as",
    "at",
    "be",
    "by",
    "did",
    "do",
    "does",
    "for",
    "from",
    "had",
    "has",
    "have",
    "her",
    "his",
    "how",
    "in",
    "is",
    "it",
    "kind",
    "kinds",
    "many",
    "of",
    "on",
    "or",
    "done",
    "some",
    "that",
    "the",
    "their",
    "this",
    "to",
    "was",
    "we",
    "what",
    "when",
    "where",
    "which",
    "who",
    "why",
    "with",
];

const MAX_QUERY_TOKENS: usize = 12;
const MAX_SEQUENCE_TOKENS: usize = 48;

pub fn normalize_query_tokens(text: &str) -> Vec<String> {
    tokenize(text, MAX_QUERY_TOKENS, true)
}

pub fn normalize_query_sequence(text: &str) -> Vec<String> {
    tokenize(text, MAX_SEQUENCE_TOKENS, false)
}

fn tokenize(text: &str, max_tokens: usize, dedup: bool) -> Vec<String> {
    let mut token = String::new();
    let mut out = Vec::new();
    let mut seen = HashSet::new();

    for ch in text.chars() {
        if ch.is_ascii_alphanumeric() || ch == '_' {
            token.push(ch.to_ascii_lowercase());
            continue;
        }

        if !token.is_empty() {
            push_token(&token, &mut out, &mut seen, dedup);
            token.clear();
            if out.len() >= max_tokens {
                return out;
            }
        }
    }

    if !token.is_empty() && out.len() < max_tokens {
        push_token(&token, &mut out, &mut seen, dedup);
    }

    out
}

pub fn build_fts_query(text: &str) -> Option<String> {
    let tokens = normalize_query_tokens(text);
    if tokens.is_empty() {
        return None;
    }
    Some(
        tokens
            .iter()
            .map(|t| format!("{t}*"))
            .collect::<Vec<String>>()
            .join(" OR "),
    )
}

fn push_token(token: &str, out: &mut Vec<String>, seen: &mut HashSet<String>, dedup: bool) {
    if token.len() < 2 || STOPWORDS.contains(&token) {
        return;
    }
    if !dedup || seen.insert(token.to_owned()) {
        out.push(token.to_owned());
    }
}

#[cfg(test)]
mod tests {
    use super::normalize_query_tokens;

    #[test]
    fn query_tokens_drop_question_fillers() {
        let tokens = normalize_query_tokens("How many children does Melanie have?");
        assert_eq!(tokens, vec!["children".to_string(), "melanie".to_string()]);
    }

    #[test]
    fn query_tokens_drop_pronoun_and_completion_fillers() {
        let tokens = normalize_query_tokens("What activities has Melanie done with her family?");
        assert_eq!(
            tokens,
            vec![
                "activities".to_string(),
                "melanie".to_string(),
                "family".to_string()
            ]
        );
    }
}
