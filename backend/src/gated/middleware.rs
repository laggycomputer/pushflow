use crate::gated::SessionUser;
use crate::ExtractedAppData;
use actix_session::SessionExt;
use actix_web::body::EitherBody;
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::http::StatusCode;
use actix_web::HttpResponse;
use entity::services;
use futures_util::future::LocalBoxFuture;
use sea_orm::ColumnTrait;
use sea_orm::EntityTrait;
use sea_orm::{PaginatorTrait, QueryFilter};
use std::future::{ready, Ready};
use std::rc::Rc;
use std::str::FromStr;
use std::task::{Context, Poll};
use uuid::Uuid;

fn bounce_no_auth<B>(req: ServiceRequest) -> ServiceResponse<EitherBody<&'static str, B>> {
    req.into_response(
        HttpResponse::with_body(StatusCode::UNAUTHORIZED, "no auth").map_into_left_body(),
    )
}

fn bounce_ambiguous<B>(req: ServiceRequest) -> ServiceResponse<EitherBody<&'static str, B>> {
    req.into_response(
        HttpResponse::with_body(StatusCode::NOT_FOUND, "bad id").map_into_left_body(),
    )
}

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
    service : S,
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
            return Box::pin(async { Ok(bounce_no_auth(req)) });
        };

        let fut = self.service.call(req);
        Box::pin(async move { fut.await.map(|ok| ok.map_into_right_body()) })
    }
}

pub struct OwnsServiceBuilder;

impl<S, B> Transform<S, ServiceRequest> for OwnsServiceBuilder
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<&'static str, B>>;
    type Error = actix_web::Error;
    type Transform = OwnsServiceMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(OwnsServiceMiddleware {
            service: Rc::new(service),
        }))
    }
}

pub struct OwnsServiceMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for OwnsServiceMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error> + 'static,
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

        let Ok(Some(user)) = session.get::<SessionUser>("user") else {
            return Box::pin(async { Ok(bounce_no_auth(req)) });
        };

        let data = req.app_data::<ExtractedAppData>().unwrap().clone();
        // gonna parse this twice overall but meh
        let try_service_id = req.match_info().get("service_id").map(Uuid::from_str);
        let service = self.service.clone();

        Box::pin(async move {
            let db = &data.clone().db;

            if let Some(Ok(service_id)) = try_service_id {
                let Ok(count) = services::Entity::find_by_id(service_id)
                    .filter(services::Column::OwnerId.eq(user.user_id))
                    .count(db)
                    .await
                else {
                    // if db err, just bounce
                    return Ok(bounce_ambiguous(req));
                };

                if count == 0 {
                    // service dne or not owned; don't reveal that
                    return Ok(bounce_ambiguous(req));
                }
            }

            service.call(req).await.map(|ok| ok.map_into_right_body())
        })
    }
}
