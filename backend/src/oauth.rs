use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
pub(crate) struct OAuth {
    pub(crate) frontend_url: String,
    pub(crate) google: GoogleOAuthConfig,
}

#[derive(Clone, Debug)]
pub(crate) struct GoogleOAuthConfig {
    pub(crate) client_id: String,
    pub(crate) client_secret: String,
}

#[derive(Serialize, Deserialize)]
struct GoogleOAuthJWT {
    // this is supposed to be a UUIDv4
    state: String,
    // UTC timestamp
    exp: usize,
}

pub(crate) mod start {
    use super::*;
    use crate::AppData;
    use actix_web::cookie::time::UtcDateTime;
    use actix_web::cookie::{Cookie, SameSite};
    use actix_web::{HttpRequest, HttpResponse, Responder, cookie, get};
    use anyhow::Context;
    use jsonwebtoken::Header;
    use std::ops::Add;

    #[get("/goog")]
    async fn goog(req: HttpRequest) -> crate::Result<impl Responder> {
        let data = *req.app_data::<&AppData>().unwrap();

        let state = uuid::Uuid::new_v4();

        let redirect_uri = format!("{}/api/login/google", data.oauth.frontend_url);

        let goog_request = data.client
        .get("https://accounts.google.com/o/oauth2/v2/auth")
        .query(&[
            ("client_id", &*data.oauth.google.client_id),
            ("redirect_uri", &*redirect_uri),
            ("response_type", "code"),
            ("state", state.to_string().as_str()),
            ("scope", "https://www.googleapis.com/auth/userinfo.email https://www.googleapis.com/auth/userinfo.profile"),
        ])
        .build()?;

        let jwt = GoogleOAuthJWT {
            state: state.to_string(),
            exp: UtcDateTime::now()
                .add(std::time::Duration::from_secs(5 * 60))
                .unix_timestamp() as usize,
        };

        let encoded = jsonwebtoken::encode(&Header::default(), &jwt, &data.jwt_keys.0)
            .context("build JWT token")?;

        // give back the google URL and the state

        Ok(HttpResponse::Ok()
            .cookie(
                Cookie::build("oauth_state", encoded.to_string())
                    .max_age(cookie::time::Duration::minutes(5))
                    // not defaulted on firefox and safari
                    .same_site(SameSite::Lax)
                    .finish(),
            )
            .body(goog_request.url().as_str().to_owned()))
    }
}

pub(crate) mod cb {
    use crate::AppData;
    use crate::gated::SessionUser;
    use actix_session::Session;
    use actix_web::http::StatusCode;
    use actix_web::{HttpRequest, HttpResponse, Responder, get};
    use anyhow::Context;
    use entity::users;
    use jsonwebtoken::Validation;
    use sea_orm::{ActiveValue, EntityTrait, sea_query};
    use serde::{Deserialize, Serialize};
    use uuid::Uuid;

    // JS will give us the query params unchanged
    #[derive(Debug, Deserialize)]
    struct OAuthCbGoogQuery {
        error: Option<String>,
        code: Option<String>,
        state: String,
    }

    #[derive(Serialize)]
    enum CompletedAuthMethod {
        Google(GoogleUserInfoResponse),
    }

    #[derive(Serialize)]
    struct CompletedAuth {
        exp: usize,
        user_id: String,
        kind: CompletedAuthMethod,
    }

    #[derive(Debug, Deserialize)]
    struct GoogleExchangeResponse {
        access_token: String,
        expires_in: usize,
        scope: String,
        // always Bearer, for now (https://developers.google.com/identity/protocols/oauth2/web-server)
        token_type: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct GoogleUserInfoResponse {
        name: String,
        picture: String,
        email: String,
        id: String,
    }

    #[get("/goog")]
    async fn goog(
        info: actix_web::web::Query<OAuthCbGoogQuery>,
        session: Session,
        req: HttpRequest,
    ) -> crate::Result<impl Responder> {
        let data = *req.app_data::<&AppData>().unwrap();

        // client has their "correct state" in the signed cookie
        let cookie_value = match req.cookie("oauth_state") {
            None => return Ok(HttpResponse::build(StatusCode::BAD_REQUEST).body("no state")),
            Some(state) => state.value().to_owned(),
        };

        let OAuthCbGoogQuery {
            code: Some(code),
            error: Option::None,
            state,
            ..
        } = info.into_inner()
        else {
            return Ok(HttpResponse::build(StatusCode::BAD_REQUEST).body("no code or error"));
        };
        let query_state = state;

        let token = jsonwebtoken::decode::<super::GoogleOAuthJWT>(
            &cookie_value,
            &data.jwt_keys.1,
            &Validation::default(),
        )?;

        if token.claims.state != query_state {
            return Ok(HttpResponse::build(StatusCode::BAD_REQUEST).body("state mismatch"));
        }

        let redirect_uri = format!("{}/api/login/google", data.oauth.frontend_url);

        let exchange_response = match data
            .client
            .post("https://oauth2.googleapis.com/token")
            .query(&[
                ("client_id", &*data.oauth.google.client_id),
                ("client_secret", &*data.oauth.google.client_secret),
                ("code", &*code),
                ("grant_type", "authorization_code"),
                ("redirect_uri", &*redirect_uri),
            ])
            .header("Content-Length", "0")
            .send()
            .await
        {
            Err(e) if e.is_status() => {
                return Ok(HttpResponse::build(StatusCode::BAD_REQUEST)
                    .body("goog did not accept exchange"));
            }
            Err(e) => {
                return Err(anyhow::anyhow!(e))?;
            }
            Ok(response) => response,
        }
        .json::<GoogleExchangeResponse>()
        .await
        .context("parse exchange response")?;

        // i don't think there's a way the user could deny scopes if we set them correctly, so let's not check again

        let userinfo_response = data
            .client
            .get("https://www.googleapis.com/userinfo/v2/me")
            .header(
                "Authorization",
                format!("Bearer {}", exchange_response.access_token),
            )
            .send()
            .await
            .context("send userinfo request to google")?
            .json::<GoogleUserInfoResponse>()
            .await
            .context("parse userinfo response")?;

        let new_user = users::ActiveModel {
            user_id: ActiveValue::Set(Uuid::now_v7()),
            goog_id: ActiveValue::Set(Some(userinfo_response.id.clone())),
            picture: ActiveValue::Set(Some(userinfo_response.picture.clone())),
        };

        let query = users::Entity::insert(new_user.clone()).on_conflict(
            sea_query::OnConflict::column(users::Column::GoogId)
                .update_column(users::Column::Picture)
                .to_owned(),
        );

        let user = query
            .exec_with_returning(&data.db)
            .await
            .context("upsert user")?;

        session.clear();
        session
            .insert(
                "user",
                SessionUser {
                    user_id: user.user_id,
                    avatar: user.picture,
                },
            )
            .context("store new session")?;

        let remove_oauth_state = {
            let mut ret = req.cookie("oauth_state").unwrap();
            ret.make_removal();
            ret
        };

        Ok(HttpResponse::build(StatusCode::OK)
            .cookie(remove_oauth_state)
            .body("hi"))
    }
}
