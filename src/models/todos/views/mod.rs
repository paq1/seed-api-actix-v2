use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Clone, ToSchema, Debug)]
pub struct TodoDataView {
    pub name: String,
    pub description: String,
    pub date: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url_image: Option<String>,
    pub flags: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, ToSchema, Debug)]
pub struct TodoCreatedView {
    #[serde(flatten)]
    pub data: TodoDataView,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "statusType")]
pub enum TodoViewState {
    #[serde(rename = "actif")]
    Todo(TodoViewCreate),
    #[serde(rename = "inactif")]
    TodoDisable(TodoViewDisable),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TodoViewCreate {
    #[serde(flatten)]
    pub data: TodoDataView,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TodoViewDisable {
    pub reason: String,
}


#[derive(Serialize, Deserialize, Clone, ToSchema, Debug)]
pub struct TodoUpdatedView {
    pub data: TodoDataView,
}

#[derive(Serialize, Deserialize, Clone, ToSchema, Debug)]
pub struct TodoDisabledView {
    pub reason: String,
}

#[derive(Serialize, Deserialize, Clone, ToSchema, Debug)]
#[serde(tag = "eventType")]
pub enum TodosViewEvent {
    #[serde(rename = "created")]
    Created(TodoCreatedView),
    #[serde(rename = "updated")]
    Updated(TodoUpdatedView),
    #[serde(rename = "disabled")]
    Disabled(TodoDisabledView),
}
