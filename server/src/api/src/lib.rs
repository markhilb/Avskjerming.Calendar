#![deny(warnings)]
#![deny(rust_2018_idioms)]

use routes::*;
use utoipa::OpenApi;

pub mod app;
pub mod error;
pub mod middlewares;
pub mod response;
pub mod routes;
pub mod settings;

#[derive(OpenApi)]
#[openapi(
    paths(
        auth::login,
        auth::logout,
        auth::logged_in,
        auth::change_password,
        employee::get_employees,
        employee::create_employee,
        employee::update_employee,
        employee::delete_employee,
        team::get_teams,
        team::create_team,
        team::update_team,
        team::delete_team,
        event::get_events,
        event::create_event,
        event::update_event,
        event::delete_event,
    ),
    components(
        schemas(
            error::ApiError,
            error::ErrorResponse,
            auth::Login,
            auth::ChangePassword,
            employee::Employee,
            employee::CreateEmployee,
            team::Team,
            team::CreateTeam,
            event::Event,
            event::CreateEvent,
            event::UpdateEvent,
        )
    ),
    tags(
        (name = "avskjerming-calendar-api")
    ))]
pub struct ApiDoc;
