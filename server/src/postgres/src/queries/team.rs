use error_stack::{IntoReport, Result, ResultExt};

use crate::{error::PostgresError, CreateTeam, PostgresAdapter, Team};

impl PostgresAdapter {
    pub async fn teams(&self) -> Result<Vec<Team>, PostgresError> {
        sqlx::query_as!(
            Team,
            r#"
SELECT
    team_id,
    "name",
    primary_color,
    secondary_color,
    disabled
FROM
    teams
                     "#
        )
        .fetch_all(&self.pool)
        .await
        .into_report()
        .change_context(PostgresError::Query)
    }

    pub async fn create_team(&self, team: CreateTeam) -> Result<i64, PostgresError> {
        let row = sqlx::query!(
            r#"
INSERT INTO
    teams ("name", primary_color, secondary_color)
VALUES
    ($1, $2, $3)
RETURNING
    team_id
            "#,
            team.name,
            team.primary_color,
            team.secondary_color,
        )
        .fetch_one(&self.pool)
        .await
        .into_report()
        .change_context(PostgresError::Query)?;

        Ok(row.team_id)
    }

    pub async fn update_team(&self, team: Team) -> Result<(), PostgresError> {
        sqlx::query!(
            r#"
UPDATE teams
SET
    "name" = $1,
    primary_color = $2,
    secondary_color = $3,
    disabled = $4
WHERE
    team_id = $5
            "#,
            team.name,
            team.primary_color,
            team.secondary_color,
            team.disabled,
            team.team_id,
        )
        .execute(&self.pool)
        .await
        .into_report()
        .change_context(PostgresError::Query)
        .map(|_| ())
    }

    pub async fn delete_team(&self, team_id: i64) -> Result<(), PostgresError> {
        sqlx::query!(
            r#"
UPDATE teams
SET
    disabled = TRUE
WHERE
    team_id = $1
            "#,
            team_id,
        )
        .execute(&self.pool)
        .await
        .into_report()
        .change_context(PostgresError::Query)
        .map(|_| ())
    }
}
