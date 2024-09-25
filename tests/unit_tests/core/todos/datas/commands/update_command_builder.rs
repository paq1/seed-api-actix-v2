use chrono::Utc;
use seed_api_actix_v2::models::todos::commands::UpdateTodoCommand;

pub struct UpdateTodoCommandBuilder {
    data: UpdateTodoCommand
}

impl UpdateTodoCommandBuilder {

    pub fn new() -> Self {
        Self {
            data: Self::data_default()
        }
    }


    pub fn build(&self) -> UpdateTodoCommand {
        self.data.clone()
    }

    pub fn with_name(&self, name: &str) -> Self {
        Self {
            data: UpdateTodoCommand {
                name: name.to_string(),
                ..self.data.clone()
            }
        }
    }


    fn data_default() -> UpdateTodoCommand {
        UpdateTodoCommand {
            name: "".to_string(),
            description: "".to_string(),
            date: Utc::now(),
            url_image: None,
            flags: vec![]
        }
    }

}