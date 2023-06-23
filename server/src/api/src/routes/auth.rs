use actix_session::Session;
use actix_web::web;
use postgres::PostgresAdapter;
use serde::{Deserialize, Serialize};
use tracing::{event, instrument, Level};
use utoipa::ToSchema;

use crate::{error::ApiError, response::Response};

#[utoipa::path(
    post,
    path = "/login",
    request_body(
        content = Login,
        content_type = "application/json",
        description = "credentials",
    ),
    responses(
        (status = 200, description = "whether the login was successfull", body = bool),
        (status = 500, description = "an internal server error occured"),
    )
)]
#[instrument(skip(session, db))]
pub async fn login(
    session: Session,
    db: web::Data<PostgresAdapter>,
    login: web::Json<Login>,
) -> Result<Response<bool>, ApiError> {
    let res = db.login(login.into_inner().password).await.map_err(|e| {
        event!(Level::ERROR, "failed to login, err: {:?}", e);
        ApiError::InternalServerError
    })?;

    if res {
        if let Err(e) = session.insert("auth", true) {
            event!(Level::ERROR, "failed to add auth to session, err: {:?}", e);
            return Err(ApiError::InternalServerError);
        }
    }

    Ok(Response::new(res))
}

#[utoipa::path(
    post,
    path = "/logout",
    responses(
        (status = 200, description = "logged out successfully"),
        (status = 500, description = "an internal server error occured"),
    )
)]
#[instrument(skip(session))]
pub async fn logout(session: Session) -> Result<Response<()>, ApiError> {
    session.remove("auth");
    Ok(Response::new(()))
}

#[utoipa::path(
    get,
    path = "/logged_in",
    responses(
        (status = 200, description = "whether the user is logged in", body = bool),
        (status = 500, description = "an internal server error occured"),
    )
)]
#[instrument(skip(session))]
pub async fn logged_in(session: Session) -> Result<Response<bool>, ApiError> {
    match session.get::<bool>("auth") {
        Ok(value) => Ok(Response::new(value.unwrap_or(false))),
        Err(e) => {
            event!(
                Level::ERROR,
                "failed to get auth from session, err: {:?}",
                e
            );
            Err(ApiError::InternalServerError)
        }
    }
}

#[utoipa::path(
    post,
    path = "/change_password",
    request_body(
        content = ChangePassword,
        content_type = "application/json",
        description = "old and new password",
    ),
    responses(
        (status = 200, description = "whether the password changed successfully", body = bool),
        (status = 500, description = "an internal server error occured"),
    )
)]
#[instrument(skip(db))]
pub async fn change_password(
    db: web::Data<PostgresAdapter>,
    body: web::Json<ChangePassword>,
) -> Result<Response<bool>, ApiError> {
    let body = body.into_inner();

    db.change_password(body.old_password, body.new_password)
        .await
        .map_err(|e| {
            event!(Level::ERROR, "failed to change password, err: {:?}", e);
            ApiError::InternalServerError
        })
        .map(Response::new)
}

#[derive(Debug, Clone, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Login {
    pub password: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ChangePassword {
    pub old_password: String,
    pub new_password: String,
}
