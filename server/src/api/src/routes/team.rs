use actix_web::web;
use postgres::PostgresAdapter;
use serde::{Deserialize, Serialize};
use tracing::{event, instrument, Level};
use utoipa::ToSchema;

use crate::{error::ApiError, response::Response};

#[utoipa::path(
    get,
    path = "/teams",
    responses(
        (status = 200, description = "all teams", body = [Team]),
        (status = 500, description = "an internal server error occured"),
    )
)]
#[instrument(skip(db))]
pub async fn get_teams(db: web::Data<PostgresAdapter>) -> Result<Response<Vec<Team>>, ApiError> {
    Ok(Response::new(
        db.teams()
            .await
            .map_err(|e| {
                event!(Level::ERROR, "failed to get teams, err: {:?}", e);
                ApiError::InternalServerError
            })?
            .into_iter()
            .map(Team::from)
            .collect(),
    ))
}

#[utoipa::path(
    post,
    path = "/teams",
    request_body(
        content = CreateTeam,
        content_type = "application/json",
        description = "the team to create",
    ),
    responses(
        (status = 200, description = "team successfully created", body = i64),
        (status = 500, description = "an internal server error occured"),
    )
)]
#[instrument(skip(db))]
pub async fn create_team(
    db: web::Data<PostgresAdapter>,
    team: web::Json<CreateTeam>,
) -> Result<Response<i64>, ApiError> {
    db.create_team(team.into_inner().into())
        .await
        .map_err(|e| {
            event!(Level::ERROR, "failed to create team, err: {:?}", e);
            ApiError::InternalServerError
        })
        .map(Response::new)
}

#[utoipa::path(
    put,
    path = "/teams",
    request_body(
        content = Team,
        content_type = "application/json",
        description = "the updated team",
    ),
    responses(
        (status = 200, description = "team successfully updated"),
        (status = 500, description = "an internal server error occured"),
    )
)]
#[instrument(skip(db))]
pub async fn update_team(
    db: web::Data<PostgresAdapter>,
    team: web::Json<Team>,
) -> Result<Response<()>, ApiError> {
    db.update_team(team.into_inner().into())
        .await
        .map_err(|e| {
            event!(Level::ERROR, "failed to update team, err: {:?}", e);
            ApiError::InternalServerError
        })
        .map(|_| Response::new(()))
}

#[utoipa::path(
    delete,
    path = "/teams/{id}",
    responses(
        (status = 200, description = "team successfully deleted"),
        (status = 500, description = "an internal server error occured"),
    )
)]
#[instrument(skip(db))]
pub async fn delete_team(
    db: web::Data<PostgresAdapter>,
    id: web::Path<i64>,
) -> Result<Response<()>, ApiError> {
    db.delete_team(id.into_inner())
        .await
        .map_err(|e| {
            event!(Level::ERROR, "failed to delete team, err: {:?}", e);
            ApiError::InternalServerError
        })
        .map(|_| Response::new(()))
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Team {
    pub id: i64,
    pub name: String,
    pub primary_color: String,
    pub secondary_color: String,
    pub disabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateTeam {
    pub name: String,
    pub primary_color: String,
    pub secondary_color: String,
}

impl From<postgres::Team> for Team {
    fn from(v: postgres::Team) -> Self {
        Self {
            id: v.team_id,
            name: v.name,
            primary_color: v.primary_color,
            secondary_color: v.secondary_color,
            disabled: v.disabled,
        }
    }
}

impl From<Team> for postgres::Team {
    fn from(v: Team) -> Self {
        Self {
            team_id: v.id,
            name: v.name,
            primary_color: v.primary_color,
            secondary_color: v.secondary_color,
            disabled: v.disabled,
        }
    }
}

impl From<CreateTeam> for postgres::CreateTeam {
    fn from(v: CreateTeam) -> Self {
        Self {
            name: v.name,
            primary_color: v.primary_color,
            secondary_color: v.secondary_color,
        }
    }
}
