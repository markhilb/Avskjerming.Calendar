use actix_web::web;
use chrono::{DateTime, Utc};
use postgres::PostgresAdapter;
use serde::{Deserialize, Serialize};
use tracing::{event, instrument, Level};
use utoipa::{IntoParams, ToSchema};

use crate::{error::ApiError, response::Response};

use super::{employee::Employee, team::Team};

#[derive(Default, Debug, Clone, Deserialize, Serialize, IntoParams)]
pub struct EventsQuery {
    #[param(value_type = Option<String>, example = "2023-01-01T00:00:00Z")]
    pub start: Option<DateTime<Utc>>,
    #[param(value_type = Option<String>, example = "2023-01-01T00:00:00Z")]
    pub end: Option<DateTime<Utc>>,
}

#[utoipa::path(
    get,
    path = "/events",
    params(EventsQuery),
    responses(
        (status = 200, description = "all events", body = [Event]),
        (status = 500, description = "an internal server error occured"),
    )
)]
#[instrument(skip(db))]
pub async fn get_events(
    db: web::Data<PostgresAdapter>,
    query: web::Query<EventsQuery>,
) -> Result<Response<Vec<Event>>, ApiError> {
    Ok(Response::new(
        db.events(query.into_inner().into())
            .await
            .map_err(|e| {
                event!(Level::ERROR, "failed to get events, err: {:?}", e);
                ApiError::InternalServerError
            })?
            .into_iter()
            .map(Event::from)
            .collect(),
    ))
}

#[utoipa::path(
    post,
    path = "/events",
    request_body(
        content = CreateEvent,
        content_type = "application/json",
        description = "the event to create",
    ),
    responses(
        (status = 200, description = "event successfully created", body = i64),
        (status = 500, description = "an internal server error occured"),
    )
)]
#[instrument(skip(db))]
pub async fn create_event(
    db: web::Data<PostgresAdapter>,
    event: web::Json<CreateEvent>,
) -> Result<Response<i64>, ApiError> {
    db.create_event(event.into_inner().into())
        .await
        .map_err(|e| {
            event!(Level::ERROR, "failed to create event, err: {:?}", e);
            ApiError::InternalServerError
        })
        .map(Response::new)
}

#[utoipa::path(
    put,
    path = "/events",
    request_body(
        content = UpdateEvent,
        content_type = "application/json",
        description = "the updated event",
    ),
    responses(
        (status = 200, description = "event successfully updated"),
        (status = 500, description = "an internal server error occured"),
    )
)]
#[instrument(skip(db))]
pub async fn update_event(
    db: web::Data<PostgresAdapter>,
    event: web::Json<UpdateEvent>,
) -> Result<Response<()>, ApiError> {
    db.update_event(event.into_inner().into())
        .await
        .map_err(|e| {
            event!(Level::ERROR, "failed to update event, err: {:?}", e);
            ApiError::InternalServerError
        })
        .map(|_| Response::new(()))
}

#[utoipa::path(
    delete,
    path = "/events/{id}",
    responses(
        (status = 200, description = "event successfully deleted"),
        (status = 500, description = "an internal server error occured"),
    )
)]
#[instrument(skip(db))]
pub async fn delete_event(
    db: web::Data<PostgresAdapter>,
    id: web::Path<i64>,
) -> Result<Response<()>, ApiError> {
    db.delete_event(id.into_inner())
        .await
        .map_err(|e| {
            event!(Level::ERROR, "failed to delete event, err: {:?}", e);
            ApiError::InternalServerError
        })
        .map(|_| Response::new(()))
}

#[derive(Debug, Clone, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    pub id: i64,
    pub title: String,
    pub details: String,
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub team: Option<Team>,
    pub employees: Vec<Employee>,
}

#[derive(Debug, Clone, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateEvent {
    pub title: String,
    pub details: String,
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub team_id: Option<i64>,
    pub employee_ids: Vec<i64>,
}

#[derive(Debug, Clone, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateEvent {
    pub id: i64,
    pub title: String,
    pub details: String,
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub team_id: Option<i64>,
    pub employee_ids: Vec<i64>,
}

impl From<postgres::Event> for Event {
    fn from(v: postgres::Event) -> Self {
        Self {
            id: v.event_id,
            title: v.title,
            details: v.details,
            start: v.start,
            end: v.end,
            team: v.team.map(Team::from),
            employees: v.employees.into_iter().map(Employee::from).collect(),
        }
    }
}

impl From<CreateEvent> for postgres::CreateEvent {
    fn from(v: CreateEvent) -> Self {
        Self {
            title: v.title,
            details: v.details,
            start: v.start,
            end: v.end,
            team_id: v.team_id,
            employee_ids: v.employee_ids,
        }
    }
}

impl From<UpdateEvent> for postgres::UpdateEvent {
    fn from(v: UpdateEvent) -> Self {
        Self {
            event_id: v.id,
            title: v.title,
            details: v.details,
            start: v.start,
            end: v.end,
            team_id: v.team_id,
            employee_ids: v.employee_ids,
        }
    }
}

impl From<EventsQuery> for postgres::EventsQuery {
    fn from(v: EventsQuery) -> Self {
        Self {
            start: v.start,
            end: v.end,
        }
    }
}

impl PartialEq for Event {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.title == other.title
            && self.details == other.details
            && self.start.timestamp_millis() == other.start.timestamp_millis()
            && self.end.timestamp_millis() == other.end.timestamp_millis()
            && self.team == other.team
            && self.employees == other.employees
    }
}

impl PartialEq for CreateEvent {
    fn eq(&self, other: &Self) -> bool {
        self.title == other.title
            && self.details == other.details
            && self.start.timestamp_millis() == other.start.timestamp_millis()
            && self.end.timestamp_millis() == other.end.timestamp_millis()
            && self.team_id == other.team_id
            && self.employee_ids == other.employee_ids
    }
}

impl PartialEq for UpdateEvent {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.title == other.title
            && self.details == other.details
            && self.start.timestamp_millis() == other.start.timestamp_millis()
            && self.end.timestamp_millis() == other.end.timestamp_millis()
            && self.team_id == other.team_id
            && self.employee_ids == other.employee_ids
    }
}

impl Event {
    pub fn to_update_event(self) -> UpdateEvent {
        UpdateEvent {
            id: self.id,
            title: self.title,
            details: self.details,
            start: self.start,
            end: self.end,
            team_id: self.team.map(|t| t.id),
            employee_ids: self.employees.into_iter().map(|e| e.id).collect(),
        }
    }
}
