use error_stack::{IntoReport, Result, ResultExt};

use crate::{error::PostgresError, PostgresAdapter};

impl PostgresAdapter {
    pub async fn login(&self, password: String) -> Result<bool, PostgresError> {
        let row = sqlx::query!(
            r#"
SELECT
    1 AS x
FROM
    passwords
WHERE
    hash = SHA512($1)
            "#,
            password.as_bytes(),
        )
        .fetch_optional(&self.pool)
        .await
        .into_report()
        .change_context(PostgresError::Query)?;

        Ok(row.is_some())
    }

    pub async fn change_password(
        &self,
        old_password: String,
        new_password: String,
    ) -> Result<bool, PostgresError> {
        let result = sqlx::query!(
            r#"
UPDATE passwords
SET
    hash = SHA512($1)
WHERE
    hash = SHA512($2)
            "#,
            new_password.as_bytes(),
            old_password.as_bytes(),
        )
        .execute(&self.pool)
        .await
        .into_report()
        .change_context(PostgresError::Query)?;

        Ok(result.rows_affected() == 1)
    }
}
