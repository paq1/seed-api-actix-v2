use crate::api::todos::todos_dbo::TodoDboState;
use crate::core::todos::data::states::todo_create::TodoCreate;
use crate::core::todos::data::states::todo_disable::TodoDisable;
use crate::core::todos::data::states::TodosStates;

impl From<TodoDboState> for TodosStates {
    fn from(value: TodoDboState) -> Self {
        match value {
            TodoDboState::TodoCreatedDbo { kind, data } =>
                TodosStates::TodoCreate(
                    TodoCreate {
                        kind,
                        data: data.into(),
                    }
                ),
            TodoDboState::TodoDisableDbo { kind, data, reason } =>
                TodosStates::TodoDisable(
                    TodoDisable {
                        kind,
                        data: data.into(),
                        reason,
                    }
                )
        }
    }
}

impl From<TodosStates> for TodoDboState {
    fn from(value: TodosStates) -> Self {
        match value {
            TodosStates::TodoCreate(data) => {
                TodoDboState::TodoCreatedDbo {
                    kind: data.kind,
                    data: data.data.into(),
                }
            }
            TodosStates::TodoDisable(data) => {
                TodoDboState::TodoDisableDbo {
                    kind: data.kind,
                    data: data.data.into(),
                    reason: data.reason,
                }
            }
        }
    }
}
