// use crate::gated::SessionUser;
// use crate::ExtractedAppData;
// use actix_session::SessionExt;
// use actix_web::body::EitherBody;
// use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
// use actix_web::http::StatusCode;
// use actix_web::HttpResponse;
// use entity::api_key_scopes;
// use entity::sea_orm_active_enums::KeyScope;
// use futures_util::future::LocalBoxFuture;
// use sea_orm::ColumnTrait;
// use sea_orm::EntityTrait;
// use sea_orm::QueryFilter;
// use std::future::{ready, Ready};
// use std::task::{Context, Poll};
// use uuid::Uuid;
//
// fn bounce_no_key<B>(req: ServiceRequest) -> ServiceResponse<EitherBody<&'static str, B>> {
//     req.into_response(
//         HttpResponse::with_body(StatusCode::UNAUTHORIZED, "no key").map_into_left_body(),
//     )
// }
// pub struct RequireKeyBuilder;
//
// impl<S, B> Transform<S, ServiceRequest> for RequireKeyMiddleware
// where
//     S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error>,
//     S::Future: 'static,
//     B: 'static,
// {
//     type Response = ServiceResponse<EitherBody<&'static str, B>>;
//     type Error = actix_web::Error;
//     type Transform = RequireKeyMiddleware;
//     type InitError = ();
//     type Future = Ready<Result<Self::Transform, Self::InitError>>;
//
//     fn new_transform(&self, service: S) -> Self::Future {
//         ready(Ok(RequireKeyMiddleware { service }))
//     }
// }
//
// pub struct RequireKeyMiddleware<S> {
//     service: S,
// }
//
// impl<S, B> Service<ServiceRequest> for RequireKeyMiddleware<S>
// where
//     S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error>,
//     S::Future: 'static,
//     B: 'static,
// {
//     type Response = ServiceResponse<EitherBody<&'static str, B>>;
//     type Error = actix_web::Error;
//     type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;
//
//     fn poll_ready(&self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
//         self.service.poll_ready(ctx)
//     }
//
//     fn call(&self, req: ServiceRequest) -> Self::Future {
//         let Some(Ok(auth)) = req.headers().get("authorization").map(|h| h.to_str()) else {
//             return Box::pin(async { Ok(bounce_no_key(req)) });
//         };
//
//         let Some(bearer) = auth.strip_prefix("Bearer: ") else {
//             return Box::pin(async { Ok(bounce_no_key(req)) });
//         };
//
//         let Ok(key) = bearer.parse::<Uuid>() else {
//             return Box::pin(async { Ok(bounce_no_key(req)) });
//         };
//
//         let Ok((service_id, group_id)) = req.match_info().load::<(Uuid, Uuid)>() else {
//             return Box::pin(async {
//                 Ok(req.into_response(
//                     HttpResponse::with_body(StatusCode::NOT_FOUND, "bad target")
//                         .map_into_left_body(),
//                 ))
//             });
//         };
//
//         let data = req.app_data::<ExtractedAppData>().unwrap().clone();
//
//         Box::pin(async move {
//             let count = api_key_scopes::Entity::find()
//                 .filter(
//                     api_key_scopes::Column::KeyId
//                         .eq(key)
//                         .and(api_key_scopes::Column::ServiceId.eq(service_id))
//                         .and(
//                             api_key_scopes::Column::GroupId
//                                 .eq(group_id)
//                                 .or(api_key_scopes::Column::GroupId.is_null()),
//                         )
//                         .and(api_key_scopes::Column::Scope.eq(KeyScope::Sub)),
//                 )
//                 .count(&data.db)
//                 .await?;
//
//             let session = req.get_session();
//
//             let Ok(Some(_)) = session.get::<SessionUser>("user") else {
//                 return Box::pin(async { Ok(crate::gated::middleware::bounce_no_auth(req)) });
//             };
//
//             let fut = self.service.call(req);
//             Box::pin(async move { fut.await.map(|ok| ok.map_into_right_body()) })
//         })
//     }
// }
