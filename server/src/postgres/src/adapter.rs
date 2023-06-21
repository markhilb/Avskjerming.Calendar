use error_stack::{IntoReport, Result, ResultExt};
use serde::Deserialize;
use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions},
    PgPool, Postgres, Transaction,
};

use crate::error::PostgresError;

#[derive(Debug, Clone)]
pub struct PostgresAdapter {
    pub(crate) pool: PgPool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PsqlSettings {
    pub ip: String,
    pub port: u16,
    pub db_name: Option<String>,
    pub username: String,
    pub password: String,
    pub max_connections: u32,
}

impl PostgresAdapter {
    pub async fn new(settings: &PsqlSettings) -> Result<Self, PostgresError> {
        let mut opts = PgConnectOptions::new()
            .username(&settings.username)
            .password(&settings.password)
            .host(&settings.ip)
            .port(settings.port);

        if let Some(db_name) = &settings.db_name {
            opts = opts.database(db_name);
        }

        let pool = PgPoolOptions::new()
            .max_connections(settings.max_connections)
            .connect_with(opts)
            .await
            .into_report()
            .change_context(PostgresError::Connection)?;

        Ok(Self { pool })
    }

    pub async fn migrate(&self) -> Result<(), PostgresError> {
        sqlx::migrate!()
            .set_ignore_missing(true)
            .run(&self.pool)
            .await
            .into_report()
            .change_context(PostgresError::Query)
    }

    pub(crate) async fn begin(&self) -> Result<Transaction<'_, Postgres>, PostgresError> {
        self.pool
            .begin()
            .await
            .into_report()
            .change_context(PostgresError::Transaction)
    }
}
