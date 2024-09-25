use framework_cqrs_lib::cqrs::core::event_sourcing::CommandHandlerCreate;
use seed_api_actix_v2::core::todos::command_handler::create_handler::TodoCreateHandler;
use seed_api_actix_v2::core::todos::data::events::TodosEvents;
use seed_api_actix_v2::models::todos::commands::TodosCommands;
use crate::unit_tests::core::todos::datas::commands::create_command_builder::CreateTodoCommandBuilder;
use crate::unit_tests::core::todos::datas::commands::disable_command_builder::DisableTodoCommandBuilder;
use crate::unit_tests::core::todos::datas::commands::update_command_builder::UpdateTodoCommandBuilder;
use crate::unit_tests::core::todos::datas::context::ContextBuilder;

#[tokio::test]
async fn create_handler_succeed_when_cmd_is_create_test() {
    // given
    let next_id = "whatever".to_string();
    let cmd = TodosCommands::Create(
        CreateTodoCommandBuilder::new()
            .with_name("test name")
            .build()
    );

    let ctx = ContextBuilder::new()
        .build();

    let handler = TodoCreateHandler {};


    // when
    let res = handler.on_command(next_id, cmd, &ctx).await;

    // then
    match res {
        Ok(event) => {
            match event {
                TodosEvents::Created(a) => {
                    assert_eq!(a.data.name, "test name".to_string());
                }
                _ => assert!(false)
            }
        }
        _ => assert!(false)
    }
}

#[tokio::test]
async fn create_handler_failed_when_cmd_is_update_test() {
    // given
    let next_id = "whatever".to_string();
    let cmd = TodosCommands::Update(
        UpdateTodoCommandBuilder::new()
            .with_name("test name")
            .build()
    );

    let ctx = ContextBuilder::new()
        .build();

    let handler = TodoCreateHandler {};


    // when
    let res = handler.on_command(next_id, cmd, &ctx).await;

    // then
    match res {
        Ok(_) => {
            assert!(false, "Unexpected event");
        }
        _ => assert!(true)
    }
}

#[tokio::test]
async fn create_handler_failed_when_cmd_is_disable_test() {
    // given
    let next_id = "whatever".to_string();
    let cmd = TodosCommands::Disable(
        DisableTodoCommandBuilder::new()
            .build()
    );

    let ctx = ContextBuilder::new()
        .build();

    let handler = TodoCreateHandler {};


    // when
    let res = handler.on_command(next_id, cmd, &ctx).await;

    // then
    match res {
        Ok(_) => {
            assert!(false, "Unexpected event");
        }
        _ => assert!(true)
    }
}

#[tokio::test]
async fn create_handler_name_should_be_create_todo() {
    // given
    let handler = TodoCreateHandler {};


    // when
    let name = handler.name();

    // then
    assert_eq!(name, "create-todo".to_string());
}
