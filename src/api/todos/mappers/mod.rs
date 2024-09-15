use crate::api::todos::todos_dbo::TodoDataDbo;
use crate::core::todos::data::todo_data::TodoData;

pub mod states;
pub mod events;

impl From<TodoDataDbo> for TodoData {
    fn from(value: TodoDataDbo) -> Self {
        Self {
            name: value.name,
            description: value.description,
            date: value.date,
            url_image: value.url_image,
            flags: value.flags,
        }
    }
}

impl From<TodoData> for TodoDataDbo {
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

