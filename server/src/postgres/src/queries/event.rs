use std::ops::Bound;

use chrono::{DateTime, Utc};
use error_stack::{IntoReport, Result, ResultExt};
use sqlx::postgres::types::PgRange;

use crate::{error::PostgresError, CreateEvent, Event, PostgresAdapter, UpdateEvent};

impl PostgresAdapter {
    pub async fn events(&self, query: EventsQuery) -> Result<Vec<Event>, PostgresError> {
        let range = match (query.start, query.end) {
            (None, None) => None,
            (start, end) => Some(PgRange {
                start: start.map(Bound::Included).unwrap_or(Bound::Unbounded),
                end: end.map(Bound::Included).unwrap_or(Bound::Unbounded),
            }),
        };

        sqlx::query!(
            r#"
SELECT
    e.event_id,
    e.title,
    e.details,
    LOWER(e.period) AS "start!",
    UPPER(e.period) AS "end!",
    CASE
        WHEN t.team_id IS NULL THEN NULL
        ELSE JSONB_BUILD_OBJECT(
            'team_id',
            t.team_id,
            'name',
            t.name,
            'primary_color',
            t.primary_color,
            'secondary_color',
            t.secondary_color,
            'disabled',
            t.disabled
        )::TEXT
    END AS team,
    COALESCE(
        JSONB_AGG(
            JSONB_BUILD_OBJECT(
                'employee_id',
                em.employee_id,
                'name',
                em.name,
                'color',
                em.color,
                'disabled',
                em.disabled
            )
        ) FILTER (
            WHERE
                em.employee_id IS NOT NULL
        ),
        '[]'
    )::TEXT AS "employees!"
FROM
    events e
    LEFT JOIN teams t ON e.team_id = t.team_id
    LEFT JOIN events__employees ee ON e.event_id = ee.event_id
    LEFT JOIN employees em ON ee.employee_id = em.employee_id
WHERE
    (
        $1::TSTZRANGE IS NULL
        OR $1 && e.period
    )
GROUP BY
    e.event_id,
    t.team_id
            "#,
            range,
        )
        .fetch_all(&self.pool)
        .await
        .into_report()
        .change_context(PostgresError::Query)?
        .into_iter()
        .map(|row| {
            Ok(Event {
                event_id: row.event_id,
                title: row.title,
                details: row.details,
                start: row.start,
                end: row.end,
                team: row
                    .team
                    .map(|t| {
                        serde_json::from_str(&t)
                            .into_report()
                            .change_context(PostgresError::DataConversion)
                            .attach_printable_lazy(|| format!("failed to deserialize team: {}", t))
                    })
                    .transpose()?,
                employees: serde_json::from_str(&row.employees)
                    .into_report()
                    .change_context(PostgresError::DataConversion)
                    .attach_printable_lazy(|| {
                        format!("failed to deserialize employees: {}", row.employees)
                    })?,
            })
        })
        .collect()
    }

    pub async fn create_event(&self, event: CreateEvent) -> Result<i64, PostgresError> {
        let mut tx = self.begin().await?;

        let row = sqlx::query!(
            r#"
INSERT INTO
    events (title, details, period, team_id)
VALUES
    ($1, $2, TSTZRANGE ($3, $4, '[]'), $5)
RETURNING
    event_id
            "#,
            event.title,
            event.details,
            event.start,
            event.end,
            event.team_id,
        )
        .fetch_one(&mut *tx)
        .await
        .into_report()
        .change_context(PostgresError::Query)?;

        sqlx::query!(
            r#"
INSERT INTO
    events__employees (event_id, employee_id)
SELECT
    $1,
    *
FROM
    UNNEST($2::BIGINT[])
            "#,
            row.event_id,
            event.employee_ids.as_slice(),
        )
        .execute(&mut *tx)
        .await
        .into_report()
        .change_context(PostgresError::Query)?;

        tx.commit()
            .await
            .into_report()
            .change_context(PostgresError::Transaction)?;

        Ok(row.event_id)
    }

    pub async fn update_event(&self, event: UpdateEvent) -> Result<(), PostgresError> {
        let mut tx = self.begin().await?;

        sqlx::query!(
            r#"
UPDATE events
SET
    title = $1,
    details = $2,
    period = TSTZRANGE ($3, $4, '[]'),
    team_id = $5
WHERE
    event_id = $6
            "#,
            event.title,
            event.details,
            event.start,
            event.end,
            event.team_id,
            event.event_id,
        )
        .execute(&mut *tx)
        .await
        .into_report()
        .change_context(PostgresError::Query)?;

        sqlx::query!(
            r#"
DELETE FROM events__employees
WHERE
    event_id = $1
            "#,
            event.event_id,
        )
        .execute(&mut *tx)
        .await
        .into_report()
        .change_context(PostgresError::Query)?;

        sqlx::query!(
            r#"
INSERT INTO
    events__employees (event_id, employee_id)
SELECT
    $1,
    *
FROM
    UNNEST($2::BIGINT[])
            "#,
            event.event_id,
            event.employee_ids.as_slice(),
        )
        .execute(&mut *tx)
        .await
        .into_report()
        .change_context(PostgresError::Query)?;

        tx.commit()
            .await
            .into_report()
            .change_context(PostgresError::Transaction)
    }

    pub async fn delete_event(&self, event_id: i64) -> Result<(), PostgresError> {
        sqlx::query!(
            r#"
DELETE FROM events
WHERE
    event_id = $1
            "#,
            event_id,
        )
        .execute(&self.pool)
        .await
        .into_report()
        .change_context(PostgresError::Query)
        .map(|_| ())
    }
}

#[derive(Debug, Clone)]
pub struct EventsQuery {
    pub start: Option<DateTime<Utc>>,
    pub end: Option<DateTime<Utc>>,
}
