use actix_cors::Cors;
use actix_session::{CookieSession, Session};
use actix_web::{
    cookie::SameSite, delete, get, middleware::Compress, post, put, web, App, HttpServer,
    Responder, Result,
};
use std::env;

use application::{
    authentication::{ChangePassword, Login},
    database::Database,
    employee::Employee,
    event::{DateRange, Event},
    logger,
    team::Team,
};

mod auth;
mod response;

use crate::auth::AuthGuard;
use crate::response::Response;

#[get("events", wrap = "AuthGuard")]
async fn get_events(
    db: web::Data<Database>,
    query: web::Query<DateRange>,
) -> Result<impl Responder> {
    response!(Database::get_events(&db, query.into_inner()).await)
}

#[post("events", wrap = "AuthGuard")]
async fn create_event(db: web::Data<Database>, event: web::Json<Event>) -> Result<impl Responder> {
    response!(Database::create_event(&db, event.into_inner()).await)
}

#[put("events", wrap = "AuthGuard")]
async fn update_event(db: web::Data<Database>, event: web::Json<Event>) -> Result<impl Responder> {
    response!(Database::update_event(&db, event.into_inner()).await)
}

#[delete("events/{id}", wrap = "AuthGuard")]
async fn delete_event(db: web::Data<Database>, path: web::Path<i64>) -> Result<impl Responder> {
    response!(Database::delete_event(&db, path.into_inner()).await)
}

#[get("teams", wrap = "AuthGuard")]
async fn get_teams(db: web::Data<Database>) -> Result<impl Responder> {
    response!(Database::get_teams(&db).await)
}

#[post("teams", wrap = "AuthGuard")]
async fn create_team(db: web::Data<Database>, team: web::Json<Team>) -> Result<impl Responder> {
    response!(Database::create_team(&db, team.into_inner()).await)
}

#[put("teams", wrap = "AuthGuard")]
async fn update_team(db: web::Data<Database>, team: web::Json<Team>) -> Result<impl Responder> {
    response!(Database::update_team(&db, team.into_inner()).await)
}

#[delete("teams/{id}", wrap = "AuthGuard")]
async fn delete_team(db: web::Data<Database>, path: web::Path<i64>) -> Result<impl Responder> {
    response!(Database::delete_team(&db, path.into_inner()).await)
}

#[get("employees", wrap = "AuthGuard")]
async fn get_employees(db: web::Data<Database>) -> Result<impl Responder> {
    response!(Database::get_employees(&db).await)
}

#[post("employees", wrap = "AuthGuard")]
async fn create_employee(
    db: web::Data<Database>,
    employee: web::Json<Employee>,
) -> Result<impl Responder> {
    response!(Database::create_employee(&db, employee.into_inner()).await)
}

#[put("employees", wrap = "AuthGuard")]
async fn update_employee(
    db: web::Data<Database>,
    employee: web::Json<Employee>,
) -> Result<impl Responder> {
    response!(Database::update_employee(&db, employee.into_inner()).await)
}

#[delete("employees/{id}", wrap = "AuthGuard")]
async fn delete_employee(db: web::Data<Database>, path: web::Path<i64>) -> Result<impl Responder> {
    response!(Database::delete_employee(&db, path.into_inner()).await)
}

#[post("login")]
async fn login(
    db: web::Data<Database>,
    form: web::Json<Login>,
    session: Session,
) -> Result<impl Responder> {
    response!(
        match Database::authenticate(&db, form.into_inner().password).await {
            Ok(res) => {
                if res {
                    match session.insert("logged_in", true) {
                        Ok(()) => Ok(true),
                        Err(e) => {
                            println!("Error: {}", e);
                            Err("An error occured during authentication".into())
                        }
                    }
                } else {
                    Ok(false)
                }
            }
            Err(e) => {
                println!("Error: {}", e);
                Err("An error occured during authentication".into())
            }
        }
    )
}

#[get("logged_in")]
async fn logged_in(session: Session) -> Result<impl Responder> {
    response!(match session.get::<bool>("logged_in") {
        Ok(val) => match val {
            Some(res) => Ok(res),
            None => {
                println!("None");
                Ok(false)
            }
        },
        Err(e) => {
            println!("Error: {}", e);
            Ok(false)
        }
    })
}

#[post("logout")]
async fn logout(session: Session) -> Result<impl Responder> {
    session.clear();
    response!(Ok(()))
}

#[post("change_password", wrap = "AuthGuard")]
async fn change_password(
    db: web::Data<Database>,
    form: web::Json<ChangePassword>,
) -> Result<impl Responder> {
    response!(Database::change_password(&db, form.into_inner()).await)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    logger::init().expect("Could not init logger");

    let db = loop {
        match Database::new(69).await {
            Ok(_db) => break _db,
            Err(e) => println!("ERROR - {}", e),
        }
    };

    Database::create_schema(&db)
        .await
        .expect("Could not create schema");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone()))
            .wrap(Compress::default())
            .wrap(
                Cors::default()
                    .supports_credentials()
                    .allow_any_header()
                    .allow_any_method()
                    .allowed_origin(if cfg!(debug_assertions) {
                        "http://129.242.219.121:4200"
                    } else {
                        "https://calendar.hilbertsen.com"
                    }),
            )
            .wrap(
                CookieSession::private(
                    env::var("SERVER_SECRET")
                        .expect("Missing enviroment variable: SERVER_SECRET")
                        .as_bytes(),
                )
                .secure(!cfg!(debug_assertions))
                .http_only(true)
                .max_age(i64::MAX)
                .same_site(if cfg!(debug_assertions) {
                    SameSite::Lax
                } else {
                    SameSite::Strict
                }),
            )
            .service(get_events)
            .service(create_event)
            .service(update_event)
            .service(delete_event)
            .service(get_teams)
            .service(create_team)
            .service(update_team)
            .service(delete_team)
            .service(get_employees)
            .service(create_employee)
            .service(update_employee)
            .service(delete_employee)
            .service(login)
            .service(logged_in)
            .service(logout)
            .service(change_password)
    })
    .bind(format!(
        "0.0.0.0:{}",
        if cfg!(debug_assertions) { "5200" } else { "80" }
    ))?
    .run()
    .await
}
