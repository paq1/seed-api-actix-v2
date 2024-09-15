use crate::core::todos::data::todo_data::TodoData;
use crate::core::todos::data::events::TodosEvents;
use crate::core::todos::data::states::TodosStates;
use crate::models::todos::views::TodoViewDisable;
use framework_cqrs_lib::cqrs::models::jsonapi::CanGetTypee;

#[derive(Clone, Debug)]
pub struct TodoDisable {
    pub kind: String,
    pub data: TodoData,
    pub reason: String,
}

impl TodoDisable {
    pub fn reduce_state(&self, event: TodosEvents) -> Option<TodosStates> {
        match event {
            _ => None // illegal transition
        }
    }
}

impl CanGetTypee for TodoDisable {
    fn get_type(&self) -> String {
        "urn:api:todos:todos".to_string()
    }
}

impl From<TodoDisable> for TodoViewDisable {
    fn from(value: TodoDisable) -> Self {
        TodoViewDisable {
            reason: value.reason,
        }
    }
}
