use crate::core::todos::data::todo_data::TodoData;
use crate::core::todos::data::events::TodosEvents;
use crate::core::todos::data::states::todo_disable::TodoDisable;
use crate::core::todos::data::states::TodosStates;
use crate::models::todos::views::{TodoDataView, TodoViewCreate};
use framework_cqrs_lib::cqrs::models::jsonapi::CanGetTypee;

#[derive(Clone, Debug)]
pub struct TodoCreate {
    pub kind: String,
    pub data: TodoData,
}

impl TodoCreate {
    pub fn reduce_state(&self, event: TodosEvents) -> Option<TodosStates> {
        match event {
            TodosEvents::Updated(updated) => Some(
                TodosStates::TodoCreate(
                    TodoCreate {
                        kind: self.kind.clone(),
                        data: updated.data.clone(),
                    }
                )
            ),
            TodosEvents::Disabled(disabled) => Some(
                TodosStates::TodoDisable(
                    TodoDisable {
                        kind: self.kind.clone(),
                        data: self.data.clone(),
                        reason: disabled.reason,
                    }
                )
            ),
            _ => None // illegal transition
        }
    }
}

impl CanGetTypee for TodoCreate {
    fn get_type(&self) -> String {
        "urn:api:todos:todos".to_string()
    }
}

impl From<TodoCreate> for TodoViewCreate {
    fn from(value: TodoCreate) -> Self {
        TodoViewCreate {
            data: value.data.into()
        }
    }
}

// MKDMKD todo voir dans quel partie impl les mappers ? (peut etre dans la partie api)
impl From<TodoData> for TodoDataView {
    fn from(value: TodoData) -> Self {
        Self {
            name: value.name,
            description: value.description,
            date: value.date,
            url_image: value.url_image,
            flags: value.flags,
        }
    }
}

