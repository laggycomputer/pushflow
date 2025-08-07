use crate::AppData;
use actix_web::cookie::{Cookie, SameSite};
use actix_web::web::Query;
use actix_web::{HttpRequest, HttpResponse, Responder, cookie, get};
use anyhow::Context;
use jsonwebtoken::Header;
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
}

#[get("/oauth/start/goog")]
async fn oauth_start_goog(req: HttpRequest) -> crate::Result<impl Responder> {
    let data = *req.app_data::<&AppData>().unwrap();

    let state = uuid::Uuid::new_v4();

    let redirect_uri = format!("{}/oauth/cb/goog", data.oauth.frontend_url);

    let goog_response = data.client
        .execute(
            data.client
                .get("https://accounts.google.com/o/oauth2/v2/auth")
                .query(&[
                    ("client_id", &*data.oauth.google.client_id),
                    // TODO: this redirect will be in the JS backend
                    ("redirect_uri", &*redirect_uri),
                    ("response_type", "code"),
                    ("state", state.to_string().as_str()),
                    ("scope", "https://www.googleapis.com/auth/userinfo.email https://www.googleapis.com/auth/userinfo.profile"),
                ])
                .build()?,
        )
        .await
        .context("init oauth with goog")?;

    let jwt = GoogleOAuthJWT {
        state: state.to_string(),
    };

    let encoded = jsonwebtoken::encode(
        &Header::default(),
        &jwt,
        &jsonwebtoken::EncodingKey::from_secret(&*data.jwt_secret),
    )
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
        .body(goog_response.url().as_str().to_owned()))
}

// JS backend will give us the query params unchanged
#[derive(Debug, Deserialize)]
struct OAuthCbGoogQuery {
    error: Option<String>,
    code: Option<String>,
}

#[get("/oauth/cb/goog")]
async fn oauth_cb_goog(info: Query<OAuthCbGoogQuery>) -> impl Responder {
    dbg!(info);
    // Google redirects user to /oauth/cb/goog?code=xxxxxxx&state=xxxxxx&...
    // client has their "correct state" in the signed cookie
    // need to give JS backend user profile URL and email

    "hi"
}
