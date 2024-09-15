use async_trait::async_trait;

use crate::core::todos::data::events::{TodoDisabled, TodosEvents};
use crate::core::todos::data::states::TodosStates;
use crate::models::todos::commands::TodosCommands;
use framework_cqrs_lib::cqrs::core::context::Context;
use framework_cqrs_lib::cqrs::core::event_sourcing::CommandHandlerUpdate;
use framework_cqrs_lib::cqrs::models::errors::{Error, ResultErr};

pub struct TodoDisableHandler;
#[async_trait]
impl CommandHandlerUpdate<TodosStates, TodosCommands, TodosEvents> for TodoDisableHandler {
    fn name(&self) -> String {
        "disable-todo".to_string()
    }

    async fn on_command(&self, _id: String, _state: TodosStates, command: TodosCommands, context: &Context) -> ResultErr<TodosEvents> {
        match command {
            TodosCommands::Disable(cmd) => Ok(
                TodosEvents::Disabled(TodoDisabled { by: context.subject.clone(), at: context.now, reason: cmd.reason })
            ),
            _ => Err(Error::Simple("bad request".to_string()))
        }
    }
}
