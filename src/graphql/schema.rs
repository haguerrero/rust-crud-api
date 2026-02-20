use async_graphql::{Schema, EmptyMutation, EmptySubscription};
use super::query::QueryRoot;
use crate::services::user_service::UserService;

pub type AppSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

pub fn create_schema(user_service: UserService) -> AppSchema {
    Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .data(user_service)
        .finish()
}
