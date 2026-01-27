use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClipItem {
    pub id: String,
    pub content: String,
    pub timestamp: i64,
    #[serde(default)]
    pub pinned: bool,
    #[serde(default)]
    pub kind: String, // "text" or "image"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryWrapper {
    pub version: u32,
    pub items: Vec<ClipItem>,
}
