use axum::body::{to_bytes, Body};
use axum::extract::{Path, State};
use axum::http::Request;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;

use crate::state::AppState;

#[derive(serde::Serialize)]
pub struct ErrorBody {
    code: &'static str,
    message: String,
}

pub struct ApiError {
    status: StatusCode,
    body: ErrorBody,
}

impl ApiError {
    fn validation(message: String) -> Self {
        Self {
            status: StatusCode::BAD_REQUEST,
            body: ErrorBody {
                code: "validation_error",
                message,
            },
        }
    }

    fn query(message: String) -> Self {
        Self {
            status: StatusCode::BAD_REQUEST,
            body: ErrorBody {
                code: "query_error",
                message,
            },
        }
    }

    fn storage(message: String) -> Self {
        Self {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            body: ErrorBody {
                code: "storage_error",
                message,
            },
        }
    }

    fn not_found(message: String) -> Self {
        Self {
            status: StatusCode::NOT_FOUND,
            body: ErrorBody {
                code: "query_error",
                message,
            },
        }
    }
}

impl From<memory_core::MemoryError> for ApiError {
    fn from(value: memory_core::MemoryError) -> Self {
        match value {
            memory_core::MemoryError::Validation(err) => Self::validation(err.to_string()),
            memory_core::MemoryError::Query(msg) => Self::query(msg),
            memory_core::MemoryError::Storage(err) => Self::storage(err.to_string()),
            memory_core::MemoryError::Serialization(err) => Self::storage(err.to_string()),
            memory_core::MemoryError::TimeParse(err) => Self::storage(err.to_string()),
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        (self.status, Json(self.body)).into_response()
    }
}

pub async fn upsert_fact(
    State(state): State<AppState>,
    Json(req): Json<memory_core::UpsertFactRequest>,
) -> Result<Json<memory_core::FactRecord>, ApiError> {
    let mut store = state.store.lock().await;
    let result = store.upsert_fact(req)?;
    Ok(Json(result))
}

pub async fn recall(
    State(state): State<AppState>,
    Json(req): Json<memory_core::RecallRequest>,
) -> Result<Json<memory_core::RecallResponse>, ApiError> {
    let store = state.store.lock().await;
    let result = store.recall(req)?;
    Ok(Json(result))
}

pub async fn list(
    State(state): State<AppState>,
    Json(req): Json<memory_core::ListRequest>,
) -> Result<Json<memory_core::RecallResponse>, ApiError> {
    let store = state.store.lock().await;
    let result = store.list(req)?;
    Ok(Json(result))
}

pub async fn forget(
    State(state): State<AppState>,
    Json(req): Json<memory_core::ForgetRequest>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let mut store = state.store.lock().await;
    store.forget(req)?;
    Ok(Json(serde_json::json!({ "ok": true })))
}

pub async fn history(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Vec<memory_core::FactVersionRecord>>, ApiError> {
    let store = state.store.lock().await;
    let result = store
        .history_by_fact_id(&id)?
        .ok_or_else(|| ApiError::not_found(format!("fact not found: {id}")))?;
    Ok(Json(result))
}

pub async fn dispatch_post(
    State(state): State<AppState>,
    request: Request<Body>,
) -> Result<Response, ApiError> {
    let path = request.uri().path().to_string();
    let payload = to_bytes(request.into_body(), usize::MAX)
        .await
        .map_err(|e| ApiError::query(e.to_string()))?;

    match path.as_str() {
        "/facts:upsert" => {
            let req: memory_core::UpsertFactRequest = parse_json(&payload)?;
            let mut store = state.store.lock().await;
            let result = store.upsert_fact(req)?;
            Ok(Json(result).into_response())
        }
        "/facts:recall" => {
            let req: memory_core::RecallRequest = parse_json(&payload)?;
            let store = state.store.lock().await;
            let result = store.recall(req)?;
            Ok(Json(result).into_response())
        }
        "/facts:list" => {
            let req: memory_core::ListRequest = parse_json(&payload)?;
            let store = state.store.lock().await;
            let result = store.list(req)?;
            Ok(Json(result).into_response())
        }
        "/facts:forget" => {
            let req: memory_core::ForgetRequest = parse_json(&payload)?;
            let mut store = state.store.lock().await;
            store.forget(req)?;
            Ok(Json(serde_json::json!({ "ok": true })).into_response())
        }
        _ => Err(ApiError::not_found(format!("unknown route: {path}"))),
    }
}

fn parse_json<T: serde::de::DeserializeOwned>(payload: &[u8]) -> Result<T, ApiError> {
    serde_json::from_slice(payload).map_err(|e| ApiError::validation(e.to_string()))
}
