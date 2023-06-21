use std::future::{ready, Ready};

use actix_session::SessionExt;
use actix_web::{
    body::EitherBody,
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpResponse,
};
use futures_util::future::LocalBoxFuture;
use tracing::{event, Level};

pub struct Auth;

impl<S, B> Transform<S, ServiceRequest> for Auth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddleware { service }))
    }
}

pub struct AuthMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        match req.get_session().get::<bool>("auth") {
            Ok(res) => match res {
                Some(true) => {
                    let fut = self.service.call(req);
                    Box::pin(async move { fut.await.map(ServiceResponse::map_into_left_body) })
                }
                _ => Box::pin(async {
                    Ok(ServiceResponse::new(
                        req.into_parts().0,
                        HttpResponse::Unauthorized().finish().map_into_right_body(),
                    ))
                }),
            },
            Err(e) => {
                event!(Level::ERROR, "failed to get cookie session, err: {:?}", e);
                Box::pin(async {
                    Ok(ServiceResponse::new(
                        req.into_parts().0,
                        HttpResponse::InternalServerError()
                            .finish()
                            .map_into_right_body(),
                    ))
                })
            }
        }
    }
}
