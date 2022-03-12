use actix_session::UserSession;
use actix_web::{
    body::BoxBody,
    dev::{forward_ready, ServiceRequest},
    http::StatusCode,
    web, HttpResponse, Result,
};
use std::future::{ready, Ready};

use actix_web::{
    dev::{Service, ServiceResponse, Transform},
    Error,
};
use futures_util::future::LocalBoxFuture;

use crate::response::Response;

macro_rules! unauthorized {
    ($req:expr) => {
        Box::pin(async move {
            Ok(ServiceResponse::new(
                $req.into_parts().0,
                HttpResponse::with_body(
                    StatusCode::OK,
                    HttpResponse::Ok()
                        .json(web::Json(Response::<usize>::error("Unauthorized".into())))
                        .into_body()
                        .into(),
                ),
            ))
        })
    };
}

pub struct AuthGuard;

impl<S, B> Transform<S, ServiceRequest> for AuthGuard
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: From<BoxBody>,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthGuardMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthGuardMiddleware { service }))
    }
}

pub struct AuthGuardMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthGuardMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: From<BoxBody>,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let session = req.get_session();
        match session.get::<bool>("logged_in") {
            Ok(res) => match res {
                Some(val) => {
                    if val {
                        let fut = self.service.call(req);
                        Box::pin(async move { fut.await })
                    } else {
                        unauthorized!(req)
                    }
                }
                None => unauthorized!(req),
            },
            Err(e) => {
                println!("{}", e);
                unauthorized!(req)
            }
        }
    }
}
