use log::error;
use serde::{Deserialize, Serialize};
use sqlx::Row;

use crate::database::Database;

#[derive(Deserialize, Serialize, sqlx::FromRow, sqlx::Type)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct Team {
    #[serde(default)]
    pub id: i64,

    pub name: String,

    #[serde(alias = "primaryColor", alias = "primary_color")]
    pub primary_color: String,

    #[serde(alias = "secondaryColor", alias = "secondary_color")]
    pub secondary_color: String,

    #[serde(default)]
    pub disabled: bool,
}

impl Database {
    pub async fn get_teams(db: &Database) -> Result<Vec<Team>, String> {
        match sqlx::query_as::<_, Team>("SELECT * FROM teams")
            .fetch_all(&db.pool)
            .await
        {
            Ok(rows) => Ok(rows),
            Err(e) => {
                error!("{}", e);
                Err("Could not get teams".into())
            }
        }
    }

    pub async fn create_team(db: &Database, team: Team) -> Result<i64, String> {
        match sqlx::query(
            "INSERT INTO teams (
                name,
                primary_color,
                secondary_color
             ) VALUES (
                $1,
                $2,
                $3
             ) RETURNING id;",
        )
        .bind(team.name)
        .bind(team.primary_color)
        .bind(team.secondary_color)
        .fetch_one(&db.pool)
        .await
        {
            Ok(row) => Ok(row.get(0)),
            Err(e) => {
                error!("{}", e);
                Err("Could not create team".into())
            }
        }
    }

    pub async fn update_team(db: &Database, team: Team) -> Result<(), String> {
        match sqlx::query(
            "UPDATE teams
             SET name = $2,
                 primary_color = $3,
                 secondary_color = $4
             WHERE id = $1",
        )
        .bind(team.id)
        .bind(team.name)
        .bind(team.primary_color)
        .bind(team.secondary_color)
        .execute(&db.pool)
        .await
        {
            Ok(res) => {
                if res.rows_affected() == 1 {
                    Ok(())
                } else {
                    Err("Could not update team".into())
                }
            }
            Err(e) => {
                error!("{}", e);
                Err("Could not update team".into())
            }
        }
    }

    pub async fn delete_team(db: &Database, id: i64) -> Result<(), String> {
        match sqlx::query(
            "UPDATE teams
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
                    Err("Could not delete team".into())
                }
            }
            Err(e) => {
                error!("{}", e);
                Err("Could not delete team".into())
            }
        }
    }
}
