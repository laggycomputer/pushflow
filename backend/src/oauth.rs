use crate::AppData;
use actix_web::cookie::time::UtcDateTime;
use actix_web::cookie::{Cookie, SameSite};
use actix_web::http::StatusCode;
use actix_web::web::Query;
use actix_web::{HttpRequest, HttpResponse, Responder, cookie, get};
use anyhow::Context;
use jsonwebtoken::{DecodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::ops::Add;
use std::time::Duration;

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

#[get("/oauth/start/goog")]
async fn oauth_start_goog(req: HttpRequest) -> crate::Result<impl Responder> {
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
            .add(Duration::from_secs(5 * 60))
            .unix_timestamp() as usize,
    };

    let encoded = jsonwebtoken::encode(
        &Header::default(),
        &jwt,
        &jsonwebtoken::EncodingKey::from_secret(&data.jwt_secret),
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
        .body(goog_request.url().as_str().to_owned()))
}

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

#[get("/oauth/cb/goog")]
async fn oauth_cb_goog(
    info: Query<OAuthCbGoogQuery>,
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

    let token = jsonwebtoken::decode::<GoogleOAuthJWT>(
        &cookie_value,
        &DecodingKey::from_secret(&data.jwt_secret),
        &Validation::default(),
    )?;

    if token.claims.state != query_state {
        return Ok(HttpResponse::build(StatusCode::BAD_REQUEST).body("state mismatch"));
    }

    let redirect_uri = format!("{}/api/login/google", data.oauth.frontend_url);

    let exp_base = UtcDateTime::now();

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
            return Ok(
                HttpResponse::build(StatusCode::BAD_REQUEST).body("goog did not accept exchange")
            );
        }
        Err(e) => {
            return Err(anyhow::anyhow!(e))?;
        }
        Ok(response) => response,
    }
    .json::<GoogleExchangeResponse>()
    .await
    .context("parse exchange response")?;

    dbg!(&exchange_response);

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

    dbg!(&userinfo_response);

    let jwt = CompletedAuth {
        // session may expire a tad early
        exp: exp_base
            .add(Duration::from_secs(exchange_response.expires_in as u64))
            .unix_timestamp() as usize,
        kind: CompletedAuthMethod::Google(userinfo_response),
    };

    let encoded = jsonwebtoken::encode(
        &Header::default(),
        &jwt,
        &jsonwebtoken::EncodingKey::from_secret(&data.jwt_secret),
    )
    .context("make cb jwt")?;

    let remove_oauth_state = {
        let mut ret = req.cookie("oauth_state").unwrap();
        ret.make_removal();
        ret
    };

    Ok(HttpResponse::build(StatusCode::OK)
        .cookie(
            Cookie::build("auth", encoded.to_string())
                .same_site(SameSite::Lax)
                .finish(),
        )
        .cookie(remove_oauth_state)
        .body("hi"))
}
