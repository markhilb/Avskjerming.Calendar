use log::error;
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::env;

#[cfg(debug_assertions)]
const PORT: &str = "localhost";

#[cfg(not(debug_assertions))]
const PORT: &str = "db";

#[derive(Clone)]
pub struct Database {
    pub pool: PgPool,
}

impl Database {
    pub async fn new(max_connection: u32) -> Result<Self, String> {
        match PgPoolOptions::new()
            .max_connections(max_connection)
            .connect(
                format!(
                    "postgres://admin:{}@{}/master",
                    env::var("DATABASE_PASSWORD")
                        .expect("Missing enviroment variable: DATABASE_PASSWORD"),
                    PORT
                )
                .as_str(),
            )
            .await
        {
            Ok(pool) => Ok(Database { pool }),
            Err(e) => {
                error!("{}", e);
                Err("Could not connect to database".into())
            }
        }
    }
}
