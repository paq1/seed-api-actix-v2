use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub enum TodosCommands {
    Create(CreateTodoCommand),
    Update(UpdateTodoCommand),
    Disable(DisableTodoCommand),
}

#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub struct CreateTodoCommand {
    pub name: String,
    pub description: String,
    pub date: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url_image: Option<String>,
    pub flags: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub struct UpdateTodoCommand {
    pub name: String,
    pub description: String,
    pub date: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url_image: Option<String>,
    pub flags: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub struct DisableTodoCommand {
    pub reason: String,
}
