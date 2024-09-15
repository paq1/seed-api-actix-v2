use async_trait::async_trait;

use crate::core::todos::data::todo_data::TodoData;
use crate::core::todos::data::events::{TodosEvents, TodoUpdated};
use crate::core::todos::data::states::TodosStates;
use crate::models::todos::commands::TodosCommands;
use framework_cqrs_lib::cqrs::core::context::Context;
use framework_cqrs_lib::cqrs::core::event_sourcing::CommandHandlerUpdate;
use framework_cqrs_lib::cqrs::models::errors::{Error, ResultErr};

pub struct TodoUpdateHandler;
#[async_trait]
impl CommandHandlerUpdate<TodosStates, TodosCommands, TodosEvents> for TodoUpdateHandler {
    fn name(&self) -> String {
        "update-todo".to_string()
    }

    async fn on_command(&self, _id: String, _state: TodosStates, command: TodosCommands, context: &Context) -> ResultErr<TodosEvents> {
        match command {
            TodosCommands::Update(c) => Ok(
                TodosEvents::Updated(
                    TodoUpdated {
                        by: context.subject.clone(),
                        at: context.now,
                        data: TodoData {
                            name: c.name,
                            description: c.description,
                            date: c.date,
                            url_image: c.url_image,
                            flags: c.flags,
                        }
                    }
                )
            ),
            _ => Err(Error::Simple("bad request".to_string()))
        }
    }
}
