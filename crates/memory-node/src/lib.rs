use std::cell::RefCell;

use napi_derive::napi;
use serde::Deserialize;

#[napi]
pub struct NativeMemoryClient {
    inner: RefCell<memory_core::MemoryStore>,
}

#[napi]
impl NativeMemoryClient {
    #[napi(constructor)]
    pub fn new(path: String) -> napi::Result<Self> {
        let store = memory_core::MemoryStore::open(&path).map_err(to_napi_error)?;
        Ok(Self {
            inner: RefCell::new(store),
        })
    }

    #[napi]
    pub fn upsert_fact(&self, payload_json: String) -> napi::Result<String> {
        let req: memory_core::UpsertFactRequest =
            serde_json::from_str(&payload_json).map_err(to_napi_error)?;
        let result = self
            .inner
            .borrow_mut()
            .upsert_fact(req)
            .map_err(to_napi_error)?;
        serde_json::to_string(&result).map_err(to_napi_error)
    }

    #[napi]
    pub fn recall(&self, payload_json: String) -> napi::Result<String> {
        let req: memory_core::RecallRequest =
            serde_json::from_str(&payload_json).map_err(to_napi_error)?;
        let result = self
            .inner
            .borrow()
            .recall(req)
            .map_err(to_napi_error)?;
        serde_json::to_string(&result).map_err(to_napi_error)
    }

    #[napi]
    pub fn list(&self, payload_json: String) -> napi::Result<String> {
        let req: memory_core::ListRequest =
            serde_json::from_str(&payload_json).map_err(to_napi_error)?;
        let result = self.inner.borrow().list(req).map_err(to_napi_error)?;
        serde_json::to_string(&result).map_err(to_napi_error)
    }

    #[napi]
    pub fn forget(&self, payload_json: String) -> napi::Result<String> {
        let req: memory_core::ForgetRequest =
            serde_json::from_str(&payload_json).map_err(to_napi_error)?;
        self.inner
            .borrow_mut()
            .forget(req)
            .map_err(to_napi_error)?;
        Ok("{\"ok\":true}".to_string())
    }

    #[napi]
    pub fn history(&self, payload_json: String) -> napi::Result<String> {
        let req: HistoryRequest = serde_json::from_str(&payload_json).map_err(to_napi_error)?;
        let result = self
            .inner
            .borrow()
            .history(&req.namespace, &req.scope_id, &req.entity, &req.attribute)
            .map_err(to_napi_error)?;
        serde_json::to_string(&result).map_err(to_napi_error)
    }
}

fn to_napi_error<E: std::fmt::Display>(err: E) -> napi::Error {
    napi::Error::from_reason(err.to_string())
}

#[derive(Deserialize)]
struct HistoryRequest {
    namespace: String,
    scope_id: String,
    entity: String,
    attribute: String,
}
