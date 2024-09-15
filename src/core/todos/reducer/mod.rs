use crate::core::todos::data::events::TodosEvents;
use crate::core::todos::data::states::TodosStates;
use framework_cqrs_lib::cqrs::core::reducer::Reducer;

pub struct TodosReducer {
    pub underlying: Reducer<TodosEvents, TodosStates>,
}

impl TodosReducer {
    pub fn new() -> Self {
        Self {
            underlying: Reducer {
                compute_new_state: |current, event| {
                    match current {
                        Some(current_state) => current_state.reduce_state(event),
                        None => TodosStates::reduce_state_from_empty(event)
                    }
                }
            }
        }
    }
}