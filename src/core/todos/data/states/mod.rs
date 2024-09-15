use crate::core::todos::data::events::TodosEvents;
use crate::core::todos::data::events::TodosEvents::Created;
use crate::core::todos::data::states::todo_disable::TodoDisable;
use crate::core::todos::data::states::todo_create::TodoCreate;
use crate::models::todos::views::TodoViewState;
use framework_cqrs_lib::cqrs::models::jsonapi::{CanBeView, CanGetTypee};

pub mod todo_create;
pub mod todo_disable;

#[derive(Clone, Debug)]
pub enum TodosStates {
    TodoCreate(TodoCreate),
    TodoDisable(TodoDisable),
}

impl TodosStates {
    pub fn reduce_state(&self, event: TodosEvents) -> Option<TodosStates> {
        match self {
            TodosStates::TodoCreate(c) => c.reduce_state(event),
            TodosStates::TodoDisable(c) => c.reduce_state(event),
        }
    }

    pub fn reduce_state_from_empty(event: TodosEvents) -> Option<TodosStates> {
        match event {
            Created(data) =>
                Some(
                    TodosStates::TodoCreate(
                        TodoCreate {
                            kind: "urn:api:todos:todos".to_string(),
                            data: data.data,
                        }
                    )
                ),
            _ => None
        }
    }
}

impl CanGetTypee for TodosStates {
    fn get_type(&self) -> String {
        match self {
            TodosStates::TodoCreate(state) => state.get_type(),
            TodosStates::TodoDisable(state) => state.get_type(),
        }
    }
}

impl CanBeView<TodoViewState> for TodosStates {
    fn to_view(&self) -> TodoViewState {
        match self.clone() {
            TodosStates::TodoCreate(state) =>
                TodoViewState::Todo(state.into()),
            TodosStates::TodoDisable(state) =>
                TodoViewState::TodoDisable(state.into())
        }
    }
}