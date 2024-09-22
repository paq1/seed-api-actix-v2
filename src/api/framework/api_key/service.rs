use crate::core::framework::api_key::service::{ApiKeyService, ImplApiKeyService};


async fn toto(apik: ImplApiKeyService) {
    let c = apik.create_api_key(&"xxx".to_string()).await;
}
