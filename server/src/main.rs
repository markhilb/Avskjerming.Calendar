use actix_cors::Cors;
use actix_web::{delete, get, post, put, web, App, HttpResponse, HttpServer, Responder, Result};

use application::{
    authentication::{ChangePassword, Login},
    database::Database,
    employee::Employee,
    event::{DateRange, Event},
    logger,
    team::Team,
};
use openssl::ssl::{SslAcceptor, SslAcceptorBuilder, SslFiletype, SslMethod};
use serde::Serialize;

macro_rules! response {
    ($res:expr) => {
        Ok(match $res {
            Ok(res) => web::Json(Response::success(res)),
            Err(e) => web::Json(Response::error(e)),
        })
    };
}

#[derive(Serialize)]
struct Response<T> {
    success: bool,
    result: Option<T>,
    error: Option<String>,
}

impl<T> Response<T> {
    pub fn success(result: T) -> Self {
        Response {
            success: true,
            result: Some(result),
            error: None,
        }
    }

    pub fn error(error: String) -> Self {
        Response {
            success: false,
            result: None,
            error: Some(error),
        }
    }
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("events")]
async fn get_events(
    db: web::Data<Database>,
    query: web::Query<DateRange>,
) -> Result<impl Responder> {
    response!(Database::get_events(&db, query.into_inner()).await)
}

#[post("events")]
async fn create_event(db: web::Data<Database>, event: web::Json<Event>) -> Result<impl Responder> {
    response!(Database::create_event(&db, event.into_inner()).await)
}

#[put("events")]
async fn update_event(db: web::Data<Database>, event: web::Json<Event>) -> Result<impl Responder> {
    response!(Database::update_event(&db, event.into_inner()).await)
}

#[delete("events/{id}")]
async fn delete_event(db: web::Data<Database>, path: web::Path<i64>) -> Result<impl Responder> {
    response!(Database::delete_event(&db, path.into_inner()).await)
}

#[get("teams")]
async fn get_teams(db: web::Data<Database>) -> Result<impl Responder> {
    response!(Database::get_teams(&db).await)
}

#[post("teams")]
async fn create_team(db: web::Data<Database>, team: web::Json<Team>) -> Result<impl Responder> {
    response!(Database::create_team(&db, team.into_inner()).await)
}

#[put("teams")]
async fn update_team(db: web::Data<Database>, team: web::Json<Team>) -> Result<impl Responder> {
    response!(Database::update_team(&db, team.into_inner()).await)
}

#[delete("teams/{id}")]
async fn delete_team(db: web::Data<Database>, path: web::Path<i64>) -> Result<impl Responder> {
    response!(Database::delete_team(&db, path.into_inner()).await)
}

#[get("employees")]
async fn get_employees(db: web::Data<Database>) -> Result<impl Responder> {
    response!(Database::get_employees(&db).await)
}

#[post("employees")]
async fn create_employee(
    db: web::Data<Database>,
    employee: web::Json<Employee>,
) -> Result<impl Responder> {
    response!(Database::create_employee(&db, employee.into_inner()).await)
}

#[put("employees")]
async fn update_employee(
    db: web::Data<Database>,
    employee: web::Json<Employee>,
) -> Result<impl Responder> {
    response!(Database::update_employee(&db, employee.into_inner()).await)
}

#[delete("employees/{id}")]
async fn delete_employee(db: web::Data<Database>, path: web::Path<i64>) -> Result<impl Responder> {
    response!(Database::delete_employee(&db, path.into_inner()).await)
}

#[post("authentication/login")]
async fn login(db: web::Data<Database>, form: web::Json<Login>) -> Result<impl Responder> {
    response!(Database::authenticate(&db, form.into_inner().password).await)
}

#[put("authentication/change_password")]
async fn change_password(
    db: web::Data<Database>,
    form: web::Json<ChangePassword>,
) -> Result<impl Responder> {
    response!(Database::change_password(&db, form.into_inner()).await)
}

fn get_ssl_builder() -> SslAcceptorBuilder {
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file(
            "/https/live/calendar.hilbertsen.com/privkey.pem",
            SslFiletype::PEM,
        )
        .expect("Could not find privkey.pem");
    builder
        .set_certificate_chain_file("/https/live/calendar.hilbertsen.com/fullchain.pem")
        .expect("Could not find fullchain.pem");
    builder
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

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone()))
            .wrap(
                Cors::default()
                    .supports_credentials()
                    .allow_any_header()
                    .allow_any_method()
                    .allowed_origin(if cfg!(debug_assertions) {
                        "http://localhost:4200"
                    } else {
                        "https://calendar.hilbertsen.com"
                    }),
            )
            .service(hello)
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
            .service(change_password)
    });

    if cfg!(debug_assertions) {
        server.bind("0.0.0.0:5200")?
    } else {
        server.bind_openssl("0.0.0.0:5200", get_ssl_builder())?
    }
    .run()
    .await
}
