use async_graphql::{Context, Object};
use crate::services::user_service::UserService;
use crate::models::user::{AuthResponse, UserResponse};

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    /// Health check — returns a greeting string.
    async fn hello(&self) -> &str {
        "Hello, world!"
    }

    /// Returns a list of all registered users.
    async fn users(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<UserResponse>> {
        let service = ctx.data::<UserService>()?;

        let users = service.get_all_users().await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;
        Ok(users)
    }

    /// Returns a single user by email address. Returns `null` if not found.
    async fn user_by_email(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "The email address to look up.")]
        email: String,
    ) -> async_graphql::Result<Option<UserResponse>> {
        let service = ctx.data::<UserService>()?;

        let user = service.get_user_by_email(&email).await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;
        Ok(user)
    }

    async fn users_paginated(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "Number of users to return. Default is 10000.")]
        limit: Option<i64>,
        #[graphql(desc = "Number of users to skip before starting to collect the result set. Default is 0.")]
        offset: Option<i64>,
    ) -> async_graphql::Result<Vec<UserResponse>> {
        let service = ctx.data::<UserService>()?;

        let users = service.get_users_paginated(limit, offset).await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;
        Ok(users)
    }

    async fn login(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "The email address of the user.")]
        email: String,
        #[graphql(desc = "The password of the user.")]
        password: String,
    ) -> async_graphql::Result<AuthResponse> {
        let service = ctx.data::<UserService>()?;

        let token = service.login(&email, &password).await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;
        Ok(AuthResponse { token })

    }
}