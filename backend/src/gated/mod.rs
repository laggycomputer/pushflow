pub(crate) mod service;

use actix_session::{Session, SessionExt};
use actix_web::body::EitherBody;
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, Responder, get};
use futures_util::future::LocalBoxFuture;
use serde::{Deserialize, Serialize};
use std::future::{Ready, ready};
use std::task::{Context, Poll};
use uuid::Uuid;

pub struct RequireAuthBuilder;

impl<S, B> Transform<S, ServiceRequest> for RequireAuthBuilder
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<&'static str, B>>;
    type Error = actix_web::Error;
    type Transform = RequireAuthMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(RequireAuthMiddleware { service }))
    }
}

pub struct RequireAuthMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for RequireAuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<&'static str, B>>;
    type Error = actix_web::Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let session = req.get_session();

        let Ok(Some(_)) = session.get::<SessionUser>("user") else {
            let res = req.into_response(
                HttpResponse::with_body(StatusCode::UNAUTHORIZED, "no auth").map_into_left_body(),
            );
            return Box::pin(async { Ok(res) });
        };

        let fut = self.service.call(req);
        Box::pin(async move { fut.await.map(|ok| ok.map_into_right_body()) })
    }
}

#[derive(Serialize, Deserialize)]
pub(crate) struct SessionUser {
    pub user_id: Uuid,
    pub avatar: Option<String>,
}

#[get("/me")]
pub async fn me(session: Session) -> crate::Result<impl Responder> {
    match session.get::<SessionUser>("user")? {
        Some(user) => Ok(HttpResponse::Ok().json(user)),
        None => Ok(HttpResponse::InternalServerError().body("no user??")),
    }
}

#[get("/logout")]
pub async fn logout(session: Session) -> crate::Result<impl Responder> {
    session.purge();

    Ok(("ok bye", StatusCode::OK))
}
