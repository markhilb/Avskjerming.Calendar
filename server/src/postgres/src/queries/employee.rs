use error_stack::{IntoReport, Result, ResultExt};

use crate::{error::PostgresError, CreateEmployee, Employee, PostgresAdapter};

impl PostgresAdapter {
    pub async fn employees(&self) -> Result<Vec<Employee>, PostgresError> {
        sqlx::query_as!(
            Employee,
            r#"
SELECT
    employee_id,
    "name",
    color,
    disabled
FROM
    employees
WHERE
    NOT disabled
                     "#
        )
        .fetch_all(&self.pool)
        .await
        .into_report()
        .change_context(PostgresError::Query)
    }

    pub async fn create_employee(&self, employee: CreateEmployee) -> Result<i64, PostgresError> {
        let row = sqlx::query!(
            r#"
INSERT INTO
    employees ("name", color)
VALUES
    ($1, $2)
RETURNING
    employee_id
            "#,
            employee.name,
            employee.color,
        )
        .fetch_one(&self.pool)
        .await
        .into_report()
        .change_context(PostgresError::Query)?;

        Ok(row.employee_id)
    }

    pub async fn update_employee(&self, employee: Employee) -> Result<(), PostgresError> {
        sqlx::query!(
            r#"
UPDATE employees
SET
    "name" = $1,
    color = $2,
    disabled = $3
WHERE
    employee_id = $4
            "#,
            employee.name,
            employee.color,
            employee.disabled,
            employee.employee_id,
        )
        .execute(&self.pool)
        .await
        .into_report()
        .change_context(PostgresError::Query)
        .map(|_| ())
    }

    pub async fn delete_employee(&self, employee_id: i64) -> Result<(), PostgresError> {
        sqlx::query!(
            r#"
UPDATE employees
SET
    disabled = TRUE
WHERE
    employee_id = $1
            "#,
            employee_id,
        )
        .execute(&self.pool)
        .await
        .into_report()
        .change_context(PostgresError::Query)
        .map(|_| ())
    }
}
