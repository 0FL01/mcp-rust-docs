#[derive(Debug, serde::Serialize)]
pub struct Item {
    pub r#type: String,
    pub href: Option<String>,
    pub path: Option<String>,
}
