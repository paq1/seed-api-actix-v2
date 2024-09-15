use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TodoDataDbo {
    pub name: String,
    pub description: String,
    pub date: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url_image: Option<String>,
    pub flags: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum TodoDboState {
    TodoCreatedDbo {
        #[serde(rename = "_kind")]
        kind: String,
        #[serde(flatten)]
        data: TodoDataDbo,
    },
    TodoDisableDbo {
        #[serde(rename = "_kind")]
        kind: String,
        #[serde(flatten)]
        data: TodoDataDbo,
        reason: String,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum TodoDboEvent {
    Created(TodoCreatedDbo),
    Updated(TodoUpdatedDbo),
    Disable(TodoDisabledDbo),
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TodoCreatedDbo {
    pub by: String,
    pub at: DateTime<Utc>,
    #[serde(flatten)]
    pub data: TodoDataDbo,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TodoUpdatedDbo {
    pub by: String,
    pub at: DateTime<Utc>,
    pub data: TodoDataDbo,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TodoDisabledDbo {
    pub by: String,
    pub at: DateTime<Utc>,
    pub reason: String,
}

