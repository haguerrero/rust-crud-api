use sqlx::MySqlPool;
use crate::graphql::schema::AppSchema;

#[derive(Clone)]
pub struct AppState {
    pub pool: MySqlPool,
    pub schema: AppSchema,
}