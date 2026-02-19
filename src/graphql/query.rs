use async_graphql::{Context, Object};
use crate::services::user_service::UserService;
use crate::models::user::UserResponse;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    
    async fn hello(&self) -> &str {
        "Hello, world!"
    }

    async fn users(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<UserResponse>> {
        let service = ctx.data::<UserService>()?;

        let users = service.get_all_users().await
            .map_err(|e| {
                async_graphql::Error::new(e.to_string())
            })?;
        Ok(users)
}
}