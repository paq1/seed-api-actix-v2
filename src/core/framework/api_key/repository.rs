use async_trait::async_trait;
use crate::core::framework::api_key::data::ApiKey;
use framework_cqrs_lib::cqrs::core::repositories::entities::RepositoryEntity;
use framework_cqrs_lib::cqrs::core::repositories::filter::{Expr, ExprGeneric, Filter, Operation};
use framework_cqrs_lib::cqrs::core::repositories::query::{PaginationDef, Query};
use framework_cqrs_lib::cqrs::models::errors::ResultErr;

#[async_trait]
pub trait ApiKeyRepository: RepositoryEntity<ApiKey, String> {
    async fn fetch_by_key(&self, key: &String) -> ResultErr<Vec<ApiKey>> {
        let query = Query {
            pagination: PaginationDef::default(),
            filter: Filter::Expr(Expr::ExprStr(ExprGeneric {
                field: "data.key".to_string(),
                operation: Operation::EqualsTo,
                head: key.to_string()
            }))
        };
        self.fetch_all(query).await.map(|entities| {
            entities.iter()
                .map(|entity| entity.data.clone())
                .collect::<Vec<_>>()
        })
    }
}
