mod oauth;

use crate::oauth::{GoogleOAuthConfig, OAuth, oauth_cb_goog, oauth_start_goog};
use actix_web::{App, HttpServer, ResponseError};
use anyhow::Context;
use std::ffi::OsString;
use std::fmt::{Display, Formatter};

#[repr(transparent)]
#[derive(Debug)]
struct AnyhowBridge(anyhow::Error);

impl Display for AnyhowBridge {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl<T> From<T> for AnyhowBridge
where
    T: Into<anyhow::Error>,
{
    fn from(value: T) -> Self {
        Self(value.into())
    }
}

type Result<T> = std::result::Result<T, AnyhowBridge>;

impl ResponseError for AnyhowBridge {
    // TODO: actual error contents
}

#[derive(Clone, Debug)]
struct AppData {
    client: reqwest::Client,
    oauth: OAuth,
    jwt_secret: Box<[u8]>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // if this fails (i.e. railway deployment, not local), meh
    let _ = dotenv::dotenv();

    let port = std::env::var("PORT")
        .unwrap_or(String::from("1451"))
        .parse::<u16>()
        .context("$PORT not valid u16 port")?;

    let app_data = Box::leak(Box::new(AppData {
        client: reqwest::Client::new(),
        oauth: OAuth {
            frontend_url: std::env::var("FRONTEND_URL").context("read env var FRONTEND_URL")?,
            google: GoogleOAuthConfig {
                client_id: std::env::var("GOOGLE_OAUTH_CLIENT_ID")
                    .context("read env var GOOGLE_OAUTH_CLIENT_ID")?,
                client_secret: std::env::var("GOOGLE_OAUTH_CLIENT_SECRET")
                    .context("read env var GOOGLE_OAUTH_CLIENT_SECRET")?,
            },
        },
        jwt_secret: Box::from(
            std::env::var_os("JWT_SECRET")
                .unwrap_or(OsString::from("0f3c13e6a2fc1e6ed08ed391de5e89276f72bb3a"))
                .as_encoded_bytes(),
        ),
    }));

    let server = {
        HttpServer::new(|| {
            App::new()
                .app_data(&*app_data)
                .service(oauth_start_goog)
                .service(oauth_cb_goog)
        })
        .bind(("127.0.0.1", port))
        .with_context(|| format!("bind to port {port}"))
    }?;

    eprintln!("ok, alive on port {port}...");

    Ok(server.run().await?)
}
