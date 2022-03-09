use log::error;
use serde::{Deserialize, Serialize};
use sqlx::Row;

use crate::database::Database;

#[derive(Deserialize, Serialize, sqlx::FromRow)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct Employee {
    #[serde(default)]
    pub id: i64,

    pub name: String,
    pub color: String,

    #[serde(default)]
    pub disabled: bool,
}

impl Database {
    pub async fn get_employees(db: &Database) -> Result<Vec<Employee>, String> {
        match sqlx::query_as::<_, Employee>("SELECT * FROM employees")
            .fetch_all(&db.pool)
            .await
        {
            Ok(rows) => Ok(rows),
            Err(e) => {
                error!("{}", e);
                Err("Could not get employees".into())
            }
        }
    }

    pub async fn create_employee(db: &Database, employee: Employee) -> Result<i64, String> {
        match sqlx::query(
            "INSERT INTO employees (
                name,
                color
             ) VALUES (
                $1,
                $2
             ) RETURNING id;",
        )
        .bind(employee.name)
        .bind(employee.color)
        .fetch_one(&db.pool)
        .await
        {
            Ok(row) => Ok(row.get(0)),
            Err(e) => {
                error!("{}", e);
                Err("Could not create employee".into())
            }
        }
    }

    pub async fn update_employee(db: &Database, employee: Employee) -> Result<(), String> {
        match sqlx::query(
            "UPDATE employees
             SET name = $2,
                 color = $3
             WHERE id = $1",
        )
        .bind(employee.id)
        .bind(employee.name)
        .bind(employee.color)
        .execute(&db.pool)
        .await
        {
            Ok(res) => {
                if res.rows_affected() == 1 {
                    Ok(())
                } else {
                    Err("Could not update employee".into())
                }
            }
            Err(e) => {
                error!("{}", e);
                Err("Could not update employee".into())
            }
        }
    }

    pub async fn delete_employee(db: &Database, id: i64) -> Result<(), String> {
        match sqlx::query(
            "UPDATE employees
             SET disabled = TRUE
             WHERE id = $1",
        )
        .bind(id)
        .execute(&db.pool)
        .await
        {
            Ok(res) => {
                if res.rows_affected() == 1 {
                    Ok(())
                } else {
                    Err("Could not delete employee".into())
                }
            }
            Err(e) => {
                error!("{}", e);
                Err("Could not delete employee".into())
            }
        }
    }
}
