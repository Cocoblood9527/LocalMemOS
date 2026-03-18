use std::sync::Mutex;

use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;
use serde::Deserialize;

#[pyclass]
struct NativeMemoryClient {
    inner: Mutex<memory_core::MemoryStore>,
}

#[pymethods]
impl NativeMemoryClient {
    #[new]
    fn new(path: String) -> PyResult<Self> {
        let store = memory_core::MemoryStore::open(&path)
            .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
        Ok(Self {
            inner: Mutex::new(store),
        })
    }

    fn upsert_fact(&self, payload_json: String) -> PyResult<String> {
        let req: memory_core::UpsertFactRequest = serde_json::from_str(&payload_json)
            .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
        let mut store = self
            .inner
            .lock()
            .map_err(|_| PyRuntimeError::new_err("memory store lock poisoned"))?;
        let result = store
            .upsert_fact(req)
            .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
        serde_json::to_string(&result).map_err(|e| PyRuntimeError::new_err(e.to_string()))
    }

    fn recall(&self, payload_json: String) -> PyResult<String> {
        let req: memory_core::RecallRequest = serde_json::from_str(&payload_json)
            .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
        let store = self
            .inner
            .lock()
            .map_err(|_| PyRuntimeError::new_err("memory store lock poisoned"))?;
        let result = store
            .recall(req)
            .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
        serde_json::to_string(&result).map_err(|e| PyRuntimeError::new_err(e.to_string()))
    }

    fn list(&self, payload_json: String) -> PyResult<String> {
        let req: memory_core::ListRequest = serde_json::from_str(&payload_json)
            .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
        let store = self
            .inner
            .lock()
            .map_err(|_| PyRuntimeError::new_err("memory store lock poisoned"))?;
        let result = store
            .list(req)
            .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
        serde_json::to_string(&result).map_err(|e| PyRuntimeError::new_err(e.to_string()))
    }

    fn forget(&self, payload_json: String) -> PyResult<String> {
        let req: memory_core::ForgetRequest = serde_json::from_str(&payload_json)
            .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
        let mut store = self
            .inner
            .lock()
            .map_err(|_| PyRuntimeError::new_err("memory store lock poisoned"))?;
        store
            .forget(req)
            .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
        Ok("{\"ok\":true}".to_string())
    }

    fn history(&self, payload_json: String) -> PyResult<String> {
        let req: HistoryRequest = serde_json::from_str(&payload_json)
            .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
        let store = self
            .inner
            .lock()
            .map_err(|_| PyRuntimeError::new_err("memory store lock poisoned"))?;
        let result = store
            .history(&req.namespace, &req.scope_id, &req.entity, &req.attribute)
            .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
        serde_json::to_string(&result).map_err(|e| PyRuntimeError::new_err(e.to_string()))
    }
}

#[pymodule]
fn _memory_sdk(_py: Python<'_>, module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_class::<NativeMemoryClient>()?;
    Ok(())
}

#[derive(Deserialize)]
struct HistoryRequest {
    namespace: String,
    scope_id: String,
    entity: String,
    attribute: String,
}
