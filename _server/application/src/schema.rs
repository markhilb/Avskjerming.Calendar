use sqlx::{postgres::PgQueryResult, Error};

use crate::database::Database;

impl Database {
    pub async fn create_schema(db: &Database) -> Result<(), Error> {
        // TODO: Find way to execute multiple statements in one query
        //       sqlx::query("..........;
        //                    ..........;
        //                    ..........;");
        //      instead of looping multiple queries
        for sql in [
            "CREATE SCHEMA IF NOT EXISTS public;",
            "CREATE TABLE IF NOT EXISTS employees (
                    id BIGSERIAL PRIMARY KEY,
                    name VARCHAR(255),
                    color VARCHAR(9),
                    disabled BOOL DEFAULT FALSE
            );",
            "CREATE TABLE IF NOT EXISTS teams (
                    id BIGSERIAL PRIMARY KEY,
                    name VARCHAR(255),
                    primary_color VARCHAR(9),
                    secondary_color VARCHAR(9),
                    disabled BOOL DEFAULT FALSE
            );",
            "CREATE TABLE IF NOT EXISTS events (
                    id BIGSERIAL PRIMARY KEY,
                    title VARCHAR(255),
                    details TEXT,
                    start TIMESTAMPTZ,
                    \"end\" TIMESTAMPTZ,
                    team_id BIGINT NULL REFERENCES teams(id),
                    employee_ids BIGINT[]
            );",
            "CREATE TABLE IF NOT EXISTS password (
                    id BIGSERIAL PRIMARY KEY,
                    hash VARCHAR(256)
            );",
            "INSERT INTO password (hash)
             SELECT $1
             WHERE NOT EXISTS (SELECT * FROM password);",
        ] {
            sqlx::query(sql)
                .bind(sha256::digest("password"))
                .execute(&db.pool)
                .await?;
        }

        Ok(())
    }

    pub async fn drop_schema(db: &Database) -> Result<PgQueryResult, Error> {
        sqlx::query("DROP SCHEMA public CASCADE")
            .execute(&db.pool)
            .await
    }
}
