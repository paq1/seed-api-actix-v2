use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApiKeyDbo {
    pub name: String,
    pub key: String,
}