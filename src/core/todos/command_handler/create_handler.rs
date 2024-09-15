use async_trait::async_trait;

use crate::core::todos::data::todo_data::TodoData;
use crate::core::todos::data::events::{TodoCreated, TodosEvents};
use crate::core::todos::data::states::TodosStates;
use crate::models::todos::commands::TodosCommands;
use framework_cqrs_lib::cqrs::core::context::Context;
use framework_cqrs_lib::cqrs::core::event_sourcing::CommandHandlerCreate;
use framework_cqrs_lib::cqrs::models::errors::{Error, ResultErr};

pub struct TodoCreateHandler {}
#[async_trait]
impl CommandHandlerCreate<TodosStates, TodosCommands, TodosEvents> for TodoCreateHandler {
    fn name(&self) -> String {
        "create-todo".to_string()
    }

    async fn on_command(&self, _id: String, command: TodosCommands, context: &Context) -> ResultErr<TodosEvents> {
        match command {
            TodosCommands::Create(c) => {
                Ok(
                    TodosEvents::Created(
                        TodoCreated {
                            by: context.clone().subject,
                            at: context.clone().now,
                            data: TodoData {
                                name: c.name,
                                description: c.description,
                                date: c.date,
                                url_image: c.url_image,
                                flags: c.flags,
                            },
                        }
                    )
                )
            }
            _ => Err(Error::Simple("bad request".to_string()))
        }
    }
}