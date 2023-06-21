use error_stack::{IntoReport, Result, ResultExt};

use crate::{error::PostgresError, PostgresAdapter, PsqlSettings};

pub struct TestDb {
    pub db: PostgresAdapter,
}

impl TestDb {
    pub async fn new(settings: &PsqlSettings) -> Result<Self, PostgresError> {
        PostgresAdapter::new(settings).await.map(|db| Self { db })
    }

    pub async fn create_db(&self, db_name: &String) -> Result<(), PostgresError> {
        sqlx::query(&format!("CREATE DATABASE \"{db_name}\" TEMPLATE postgres"))
            .execute(&self.db.pool)
            .await
            .into_report()
            .change_context(PostgresError::Query)
            .map(|_| ())
    }

    pub async fn drop_db(&self, db_name: &String) -> Result<(), PostgresError> {
        sqlx::query(&format!("DROP DATABASE \"{db_name}\" WITH (FORCE)"))
            .execute(&self.db.pool)
            .await
            .into_report()
            .change_context(PostgresError::Query)?;

        self.db.pool.close().await;

        Ok(())
    }
}
