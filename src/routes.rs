use axum::{
    Router,
    routing::{get, post, put, delete},
    response::{Html, IntoResponse},
};
use axum::extract::State;
use sqlx::MySqlPool;
use tower_http::compression::CompressionLayer;
use crate::handlers::health;
use crate::handlers::user_handler;
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use async_graphql::http::GraphiQLSource;
use crate::services::user_service::UserService;
use crate::AppState;
use crate::graphql::schema::create_schema;

pub fn create_router(pool: MySqlPool) -> Router {
    let user_service = UserService::new(pool.clone());
    let schema = create_schema(user_service);

    let state = AppState {
        pool,
        schema,
    };
    
    Router::new()
        .route("/health", get(health::health))
        .route("/users", get(user_handler::get_users))
        .route("/users", post(user_handler::create_user))
        .route("/auth/login", post(user_handler::login))
        .route("/users/email", get(user_handler::get_user_by_email))
        .route("/users/email", put(user_handler::update_user))
        .route("/users/email", delete(user_handler::delete_user))
        .route("/graphql", post(graphql_handler))
        .route("/playground", get(graphql_playground))
        .layer(CompressionLayer::new())
        .with_state(state)
}

async fn graphql_handler(
    State(state): State<AppState>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    state.schema.execute(req.into_inner()).await.into()
}

async fn graphql_playground() -> impl IntoResponse {
    Html(GraphiQLSource::build().endpoint("/graphql").finish())
}