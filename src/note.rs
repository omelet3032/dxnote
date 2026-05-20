#[derive(Clone, PartialEq, Debug)]
pub struct NoteSummary {
    pub id: i64,
    pub content: String,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
