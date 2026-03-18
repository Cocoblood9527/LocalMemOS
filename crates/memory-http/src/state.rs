#[derive(Clone)]
pub struct AppState {
    pub store_path: String,
    pub store: std::sync::Arc<tokio::sync::Mutex<memory_core::MemoryStore>>,
}

impl AppState {
    pub fn new(path: &str) -> Result<Self, memory_core::MemoryError> {
        let store = memory_core::MemoryStore::open(path)?;
        Ok(Self {
            store_path: path.to_string(),
            store: std::sync::Arc::new(tokio::sync::Mutex::new(store)),
        })
    }
}
