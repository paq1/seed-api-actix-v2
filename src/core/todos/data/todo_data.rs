use chrono::{DateTime, Utc};

#[derive(Clone, Debug)]
pub struct TodoData {
    pub name: String,
    pub description: String,
    pub date: DateTime<Utc>,
    pub url_image: Option<String>,
    pub flags: Vec<String>,
}
