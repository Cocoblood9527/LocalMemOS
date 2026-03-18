#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UpsertFactRequest {
    pub namespace: String,
    pub scope_id: String,
    pub entity: String,
    pub attribute: String,
    pub value: serde_json::Value,
    pub confidence: Option<f32>,
    pub tags: Vec<String>,
    pub valid_from: Option<chrono::DateTime<chrono::Utc>>,
    pub source_kind: String,
    pub source_ref: Option<String>,
    pub evidence_summary: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RecallRequest {
    pub namespace: String,
    pub scope_id: String,
    pub entity: Option<String>,
    pub attribute: Option<String>,
    pub text_query: Option<String>,
    // Reserved in v1. Kept for compatibility; current core recall semantics ignore this flag.
    pub include_history: bool,
    pub as_of: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ListRequest {
    pub namespace: String,
    pub scope_id: String,
    pub entity: Option<String>,
    pub attribute: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ForgetRequest {
    pub namespace: String,
    pub scope_id: String,
    pub entity: String,
    pub attribute: String,
}

impl UpsertFactRequest {
    pub fn validate(&self) -> Result<(), crate::error::ValidationError> {
        if self.namespace.trim().is_empty() {
            return Err(crate::error::ValidationError::MissingField("namespace"));
        }
        if self.scope_id.trim().is_empty() {
            return Err(crate::error::ValidationError::MissingField("scope_id"));
        }
        if self.entity.trim().is_empty() {
            return Err(crate::error::ValidationError::MissingField("entity"));
        }
        if self.attribute.trim().is_empty() {
            return Err(crate::error::ValidationError::MissingField("attribute"));
        }
        if self.source_kind.trim().is_empty() {
            return Err(crate::error::ValidationError::MissingField("source_kind"));
        }
        if let Some(confidence) = self.confidence {
            if !(0.0..=1.0).contains(&confidence) {
                return Err(crate::error::ValidationError::InvalidConfidence);
            }
        }
        Ok(())
    }

    pub fn manual(
        namespace: &str,
        scope_id: &str,
        entity: &str,
        attribute: &str,
        value: &str,
    ) -> Self {
        Self {
            namespace: namespace.into(),
            scope_id: scope_id.into(),
            entity: entity.into(),
            attribute: attribute.into(),
            value: serde_json::json!(value),
            confidence: Some(1.0),
            tags: vec![],
            valid_from: None,
            source_kind: "manual".into(),
            source_ref: Some("test".into()),
            evidence_summary: None,
        }
    }
}

impl RecallRequest {
    pub fn exact(namespace: &str, scope_id: &str, entity: &str, attribute: &str) -> Self {
        Self {
            namespace: namespace.into(),
            scope_id: scope_id.into(),
            entity: Some(entity.into()),
            attribute: Some(attribute.into()),
            text_query: None,
            include_history: false,
            as_of: None,
        }
    }

    pub fn text(namespace: &str, scope_id: &str, text_query: &str) -> Self {
        Self {
            namespace: namespace.into(),
            scope_id: scope_id.into(),
            entity: None,
            attribute: None,
            text_query: Some(text_query.into()),
            include_history: false,
            as_of: None,
        }
    }
}
