use seed_api_actix_v2::models::todos::commands::DisableTodoCommand;

pub struct DisableTodoCommandBuilder {
    data: DisableTodoCommand
}

impl DisableTodoCommandBuilder {

    pub fn new() -> Self {
        Self {
            data: Self::data_default()
        }
    }


    pub fn build(&self) -> DisableTodoCommand {
        self.data.clone()
    }

    pub fn with_reason(&self, reason: &str) -> Self {
        Self {
            data: DisableTodoCommand {
                reason: reason.to_string(),
                ..self.data.clone()
            }
        }
    }


    fn data_default() -> DisableTodoCommand {
        DisableTodoCommand {
            reason: "".to_string()
        }
    }

}