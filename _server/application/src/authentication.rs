use log::error;
use serde::Deserialize;
use sqlx::Row;

use crate::database::Database;

#[derive(Deserialize)]
pub struct Login {
    pub password: String,
}

#[derive(Deserialize)]
pub struct ChangePassword {
    old: String,
    new: String,
}

impl Database {
    pub async fn authenticate(db: &Database, password: String) -> Result<bool, String> {
        match sqlx::query("SELECT hash FROM password")
            .fetch_one(&db.pool)
            .await
        {
            Ok(row) => Ok(sha256::digest(password) == row.get::<String, usize>(0)),
            Err(e) => {
                error!("{}", e);
                Err("Could not get password hash".into())
            }
        }
    }

    pub async fn change_password(db: &Database, form: ChangePassword) -> Result<bool, String> {
        if Database::authenticate(db, form.old).await? {
            match sqlx::query("UPDATE password SET hash = $1")
                .bind(sha256::digest(form.new))
                .execute(&db.pool)
                .await
            {
                Ok(res) => {
                    if res.rows_affected() == 1 {
                        Ok(true)
                    } else {
                        Err("Could not change password".into())
                    }
                }
                Err(e) => {
                    error!("{}", e);
                    Err("Could not change password".into())
                }
            }
        } else {
            Ok(false)
        }
    }
}
