#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Failed to initialize client: {0}")]
    InitializeClient(String),

    #[error("Network error: {0}")]
    CratesIoApi(String),
}

impl Into<rmcp::ErrorData> for Error {
    fn into(self) -> rmcp::ErrorData {
        rmcp::ErrorData::new(
            rmcp::model::ErrorCode(1),
            self.to_string(),
            Some(rmcp::serde_json::Value::String(self.to_string())),
        )
    }
}
