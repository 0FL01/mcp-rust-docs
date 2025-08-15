#[derive(Debug, serde::Serialize)]
pub struct CrateSummaryEntity {
    pub name: String,
    pub description: Option<String>,
    pub latest_stable_version: Option<String>,
    pub latest_version: String,
    pub downloads: u64,
    pub created_at: String,
    pub updated_at: String,
}
