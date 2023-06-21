CREATE TABLE
    employees (
        employee_id BIGSERIAL PRIMARY KEY,
        "name" TEXT NOT NULL,
        color TEXT NOT NULL,
        disabled BOOLEAN NOT NULL DEFAULT FALSE
    );

CREATE TABLE
    teams (
        team_id BIGSERIAL PRIMARY KEY,
        "name" TEXT NOT NULL,
        primary_color TEXT NOT NULL,
        secondary_color TEXT NOT NULL,
        disabled BOOLEAN NOT NULL DEFAULT FALSE
    );

CREATE TABLE
    events (
        event_id BIGSERIAL PRIMARY KEY,
        "title" TEXT NOT NULL,
        details TEXT NOT NULL,
        period TSTZRANGE NOT NULL CHECK (
            NOT ISEMPTY(period)
            AND NOT LOWER_INF(period)
            AND NOT UPPER_INF(period)
        ),
        team_id BIGINT REFERENCES teams (team_id) ON DELETE CASCADE
    );

CREATE INDEX ON events USING GIST (period);

CREATE INDEX ON events (team_id);

CREATE TABLE
    events__employees (
        event_id BIGINT REFERENCES events (event_id) ON DELETE CASCADE,
        employee_id BIGINT REFERENCES employees (employee_id) ON DELETE CASCADE,
        PRIMARY KEY (event_id, employee_id)
    );

CREATE TABLE
    passwords (hash BYTEA NOT NULL);

INSERT INTO
    passwords (hash)
VALUES
    (SHA512('password'));
