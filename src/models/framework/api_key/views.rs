use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Clone, ToSchema, Debug)]
pub struct ApiKeyView {
    pub name: String,
    pub key: String,
}
