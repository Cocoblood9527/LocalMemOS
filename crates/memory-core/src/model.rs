#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FactRecord {
    pub id: String,
    pub namespace: String,
    pub scope_id: String,
    pub entity: String,
    pub attribute: String,
    pub value_json: serde_json::Value,
    pub value_text: Option<String>,
    pub confidence: Option<f32>,
    pub valid_from: chrono::DateTime<chrono::Utc>,
    pub valid_to: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FactVersionRecord {
    pub id: String,
    pub fact_id: String,
    pub namespace: String,
    pub scope_id: String,
    pub entity: String,
    pub attribute: String,
    pub value_json: serde_json::Value,
    pub value_text: Option<String>,
    pub confidence: Option<f32>,
    pub valid_from: chrono::DateTime<chrono::Utc>,
    pub valid_to: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RecallResponse {
    pub facts: Vec<FactRecord>,
}
