use chrono::Utc;
use seed_api_actix_v2::core::todos::data::todo_data::TodoData;

pub mod create_command_builder;
pub mod update_command_builder;
pub mod disable_command_builder;


pub struct TodoDataBuilder {
    data: TodoData
}

impl TodoDataBuilder {

    pub fn new() -> Self {
        Self {
            data: Self::data_default()
        }
    }

    pub fn build(&self) -> TodoData {
        self.data.clone()
    }

    pub fn with_name(&self, name: &str) -> Self {
        Self {
            data: TodoData {
                name: name.to_string(),
                ..self.data.clone()
            }
        }
    }

    fn data_default() -> TodoData {
        TodoData {
            name: "".to_string(),
            description: "".to_string(),
            date: Utc::now(),
            url_image: None,
            flags: vec![]
        }
    }
}