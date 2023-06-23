use std::{io, net::TcpListener};

use actix_cors::Cors;
use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::{
    cookie::Key,
    dev::Server,
    middleware::{Compress, Condition},
    web::{self, Data},
    HttpServer,
};
use postgres::PostgresAdapter;
use tracing_actix_web::TracingLogger;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::{
    middlewares, routes,
    settings::{Environment, Settings},
    ApiDoc,
};

pub struct App {
    server: Server,
    port: u16,
}

impl App {
    pub async fn new(settings: &Settings) -> Self {
        let postgres = PostgresAdapter::new(&settings.postgres).await.unwrap();

        let environment = settings.environment;

        if environment != Environment::Test {
            postgres.migrate().await.unwrap();
        }

        let listener = TcpListener::bind(settings.api.listener_address()).unwrap();
        let port = listener.local_addr().unwrap().port();

        let secret_key = settings.api.secret_key.clone();

        let server = HttpServer::new(move || {
            let mut cors = Cors::default()
                .allow_any_header()
                .allow_any_method()
                .supports_credentials();

            cors = match environment {
                Environment::Td => cors.allowed_origin("https://calendar.hilbertsen.com"),
                Environment::Test | Environment::Local | Environment::Avskjerming => {
                    cors.allow_any_origin()
                }
            };

            actix_web::App::new()
                .app_data(Data::new(postgres.clone()))
                .wrap(Compress::default())
                .wrap(cors)
                .wrap(TracingLogger::default())
                .wrap(Condition::new(
                    secret_key.is_some(),
                    SessionMiddleware::new(
                        CookieSessionStore::default(),
                        secret_key
                            .as_ref()
                            .map(|key| Key::derive_from(key.as_bytes()))
                            .unwrap_or_else(Key::generate),
                    ),
                ))
                .service(
                    SwaggerUi::new("/swagger-ui/{_:.*}")
                        .url("/api-doc/openapi.json", ApiDoc::openapi()),
                )
                .route("/login", web::post().to(routes::auth::login))
                .route("/logout", web::post().to(routes::auth::logout))
                .route("/logged_in", web::get().to(routes::auth::logged_in))
                .service(
                    web::scope("")
                        .wrap(Condition::new(secret_key.is_some(), middlewares::Auth))
                        .route(
                            "/change_password",
                            web::post().to(routes::auth::change_password),
                        )
                        .route("/employees", web::get().to(routes::employee::get_employees))
                        .route(
                            "/employees",
                            web::post().to(routes::employee::create_employee),
                        )
                        .route(
                            "/employees",
                            web::put().to(routes::employee::update_employee),
                        )
                        .route(
                            "/employees/{id}",
                            web::delete().to(routes::employee::delete_employee),
                        )
                        .route("/teams", web::get().to(routes::team::get_teams))
                        .route("/teams", web::post().to(routes::team::create_team))
                        .route("/teams", web::put().to(routes::team::update_team))
                        .route("/teams/{id}", web::delete().to(routes::team::delete_team))
                        .route("/events", web::get().to(routes::event::get_events))
                        .route("/events", web::post().to(routes::event::create_event))
                        .route("/events", web::put().to(routes::event::update_event))
                        .route(
                            "/events/{id}",
                            web::delete().to(routes::event::delete_event),
                        ),
                )
        })
        .listen(listener)
        .unwrap()
        .run();

        Self { server, port }
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run(self) -> Result<(), io::Error> {
        self.server.await
    }
}
