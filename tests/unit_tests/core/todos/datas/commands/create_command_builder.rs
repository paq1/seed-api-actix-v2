use chrono::Utc;
use seed_api_actix_v2::models::todos::commands::CreateTodoCommand;

pub struct CreateTodoCommandBuilder {
    data: CreateTodoCommand
}

impl CreateTodoCommandBuilder {

    pub fn new() -> Self {
        Self {
            data: Self::data_default()
        }
    }


    pub fn build(&self) -> CreateTodoCommand {
        self.data.clone()
    }

    pub fn with_name(&self, name: &str) -> Self {
        Self {
            data: CreateTodoCommand {
                name: name.to_string(),
                ..self.data.clone()
            }
        }
    }


    fn data_default() -> CreateTodoCommand {
        CreateTodoCommand {
            name: "".to_string(),
            description: "".to_string(),
            date: Utc::now(),
            url_image: None,
            flags: vec![]
        }
    }

}