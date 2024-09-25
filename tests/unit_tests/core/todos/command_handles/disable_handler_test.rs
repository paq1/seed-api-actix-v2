use crate::unit_tests::core::todos::datas::commands::create_command_builder::CreateTodoCommandBuilder;
use crate::unit_tests::core::todos::datas::commands::disable_command_builder::DisableTodoCommandBuilder;
use crate::unit_tests::core::todos::datas::commands::update_command_builder::UpdateTodoCommandBuilder;
use crate::unit_tests::core::todos::datas::commands::TodoDataBuilder;
use crate::unit_tests::core::todos::datas::context::ContextBuilder;
use framework_cqrs_lib::cqrs::core::event_sourcing::CommandHandlerUpdate;
use seed_api_actix_v2::core::todos::command_handler::disable_handler::TodoDisableHandler;
use seed_api_actix_v2::core::todos::data::events::TodosEvents;
use seed_api_actix_v2::core::todos::data::states::todo_create::TodoCreate;
use seed_api_actix_v2::core::todos::data::states::TodosStates;
use seed_api_actix_v2::models::todos::commands::TodosCommands;

#[tokio::test]
async fn disable_handler_succeed_when_cmd_is_disable_test() {
    // given
    let next_id = "whatever".to_string();
    let current_state = TodosStates::TodoCreate(TodoCreate {
        kind: "whatever".to_string(),
        data: TodoDataBuilder::new().build()
    });
    let cmd = TodosCommands::Disable(
        DisableTodoCommandBuilder::new()
            .with_reason("no reason ... pouet")
            .build()
    );
    let ctx = ContextBuilder::new()
        .build();

    let handler = TodoDisableHandler {};

    // when
    let res = handler.on_command(next_id, current_state, cmd, &ctx).await;

    // then
    match res {
        Ok(event) => {
            match event {
                TodosEvents::Disabled(a) => {
                    assert_eq!(a.reason, "no reason ... pouet".to_string());
                }
                _ => assert!(false)
            }
        }
        _ => assert!(false)
    }
}

#[tokio::test]
async fn disable_handler_failed_when_cmd_is_update_test() {
    // given
    let next_id = "whatever".to_string();
    let current_state = TodosStates::TodoCreate(TodoCreate {
        kind: "whatever".to_string(),
        data: TodoDataBuilder::new().build()
    });
    let cmd = TodosCommands::Update(
        UpdateTodoCommandBuilder::new()
            .with_name("test name")
            .build()
    );

    let ctx = ContextBuilder::new()
        .build();

    let handler = TodoDisableHandler {};

    // when
    let res = handler.on_command(next_id, current_state, cmd, &ctx).await;

    // then
    match res {
        Ok(_) => {
            assert!(false, "Unexpected event");
        }
        _ => assert!(true)
    }
}

#[tokio::test]
async fn disable_handler_failed_when_cmd_is_create_test() {
    // given
    let next_id = "whatever".to_string();
    let current_state = TodosStates::TodoCreate(TodoCreate {
        kind: "whatever".to_string(),
        data: TodoDataBuilder::new().build()
    });
    let cmd = TodosCommands::Create(
        CreateTodoCommandBuilder::new()
            .build()
    );

    let ctx = ContextBuilder::new()
        .build();

    let handler = TodoDisableHandler {};

    // when
    let res = handler.on_command(next_id, current_state, cmd, &ctx).await;

    // then
    match res {
        Ok(_) => {
            assert!(false, "Unexpected event");
        }
        _ => assert!(true)
    }
}

#[tokio::test]
async fn disable_handler_name_should_be_disable_todo() {
    // given
    let handler = TodoDisableHandler {};

    // when
    let name = handler.name();

    // then
    assert_eq!(name, "disable-todo".to_string());
}
