use axum::{
    Router,
    routing::{get, post, put, delete},
    response::{Html, IntoResponse},
    Extension,
};
use sqlx::MySqlPool;
use tower_http::compression::CompressionLayer;

use crate::handlers::health;
use crate::handlers::user_handler;

use async_graphql_axum::{GraphQL, GraphQLRequest, GraphQLResponse};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use crate::graphql::schema::{create_schema, AppSchema};



pub fn create_router(pool: MySqlPool) -> Router {
    let schema = create_schema();

    Router::new()
        .route("/health", get(health::health))
        .route("/users", get(user_handler::get_users))
        .route("/users", post(user_handler::create_user))
        .route("/auth/login", post(user_handler::login))
        .route("/users/email", get(user_handler::get_user_by_email))
        .route("/users/email", put(user_handler::update_user))
        .route("/users/email", delete(user_handler::delete_user))
        .route("/graphql", get(graphql_playground).post(graphql_handler))
        .layer(CompressionLayer::new())
        .layer(Extension(schema))
        .with_state(pool)
}

async fn graphql_handler(
    Extension(schema): Extension<AppSchema>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
}

