use chrono::{DateTime, Utc};
use log::error;
use serde::{Deserialize, Serialize};
use sqlx::Row;

use crate::{database::Database, employee::Employee, team::Team};

#[derive(Deserialize, Serialize, sqlx::FromRow)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct Event {
    #[serde(default)]
    pub id: i64,

    pub title: String,
    pub details: String,
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,

    #[serde(default)]
    pub team: Option<sqlx::types::Json<Team>>,

    #[serde(default)]
    pub employees: sqlx::types::Json<Vec<Employee>>,
}

#[derive(Deserialize)]
pub struct DateRange {
    from: Option<DateTime<Utc>>,
    to: Option<DateTime<Utc>>,
}

impl Database {
    pub async fn get_events(db: &Database, query: DateRange) -> Result<Vec<Event>, String> {
        let sql = format!(
            "SELECT e.*,
                    ROW_TO_JSON(t.*) team,
                    CASE COUNT(em)
                        WHEN 0 THEN '[]'
                        ELSE JSON_AGG(ROW_TO_JSON(em.*))
                    END employees
             FROM events e
             LEFT JOIN teams t ON t.id = e.team_id
             LEFT JOIN employees em ON em.id = ANY(e.employee_ids)
             {}
             GROUP BY e.id, t.id",
            if query.from.is_some() && query.to.is_some() {
                "WHERE e.start BETWEEN $1 AND $2
                 OR e.\"end\" BETWEEN $1 AND $2
                 OR (e.start < $1 AND e.\"end\" > $2)"
            } else if query.from.is_some() {
                "WHERE e.start >= $1 OR e.\"end\" >= $1"
            } else if query.to.is_some() {
                "WHERE e.start <= $2 OR e.\"end\" <= $2"
            } else {
                ""
            }
        );

        match sqlx::query_as::<_, Event>(&sql)
            .bind(query.from)
            .bind(query.to)
            .fetch_all(&db.pool)
            .await
        {
            Ok(rows) => Ok(rows),
            Err(e) => {
                error!("{}", e);
                Err("Could not get events".into())
            }
        }
    }

    pub async fn create_event(db: &Database, event: Event) -> Result<i64, String> {
        match sqlx::query(
            "INSERT INTO events (
                title,
                details,
                start,
                \"end\",
                team_id,
                employee_ids
             ) VALUES (
                $1,
                $2,
                $3,
                $4,
                $5,
                $6
             ) RETURNING id;",
        )
        .bind(event.title)
        .bind(event.details)
        .bind(event.start)
        .bind(event.end)
        .bind(match event.team {
            Some(team) => Some(team.id),
            None => None,
        })
        .bind(event.employees.iter().map(|e| e.id).collect::<Vec<i64>>())
        .fetch_one(&db.pool)
        .await
        {
            Ok(row) => Ok(row.get(0)),
            Err(e) => {
                error!("{}", e);
                Err("Could not create event".into())
            }
        }
    }

    pub async fn update_event(db: &Database, event: Event) -> Result<(), String> {
        match sqlx::query(
            "UPDATE events
             SET title = $2,
                 details = $3,
                 start = $4,
                 \"end\" = $5,
                 team_id = $6,
                 employee_ids = $7
             WHERE id = $1",
        )
        .bind(event.id)
        .bind(event.title)
        .bind(event.details)
        .bind(event.start)
        .bind(event.end)
        .bind(match event.team {
            Some(team) => Some(team.id),
            None => None,
        })
        .bind(event.employees.iter().map(|e| e.id).collect::<Vec<i64>>())
        .execute(&db.pool)
        .await
        {
            Ok(res) => {
                if res.rows_affected() == 1 {
                    Ok(())
                } else {
                    Err("Could not update event".into())
                }
            }
            Err(e) => {
                error!("{}", e);
                Err("Could not update event".into())
            }
        }
    }

    pub async fn delete_event(db: &Database, id: i64) -> Result<(), String> {
        match sqlx::query("DELETE FROM events WHERE id = $1")
            .bind(id)
            .execute(&db.pool)
            .await
        {
            Ok(res) => {
                if res.rows_affected() == 1 {
                    Ok(())
                } else {
                    Err("Could not delete event".into())
                }
            }
            Err(e) => {
                error!("{}", e);
                Err("Could not delete event".into())
            }
        }
    }
}
