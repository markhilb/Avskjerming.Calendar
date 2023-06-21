use chrono::{DateTime, Utc};

use crate::{Employee, Team};

#[derive(Debug, Clone)]
pub struct Event {
    pub event_id: i64,
    pub title: String,
    pub details: String,
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub team: Option<Team>,
    pub employees: Vec<Employee>,
}

#[derive(Debug, Clone)]
pub struct CreateEvent {
    pub title: String,
    pub details: String,
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub team_id: Option<i64>,
    pub employee_ids: Vec<i64>,
}

#[derive(Debug, Clone)]
pub struct UpdateEvent {
    pub event_id: i64,
    pub title: String,
    pub details: String,
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub team_id: Option<i64>,
    pub employee_ids: Vec<i64>,
}
