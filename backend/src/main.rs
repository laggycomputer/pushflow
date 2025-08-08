mod gated;
mod oauth;

use crate::oauth::{GoogleOAuthConfig, OAuth};
use actix_session::storage::{RedisSessionStore, SessionStore};
use actix_session::SessionMiddleware;
use actix_web::cookie::Key;
use actix_web::{App, HttpServer, ResponseError};
use anyhow::Context;
use deadpool_redis::{Config, Runtime};
use migration::{Migrator, MigratorTrait};
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

#[derive(Clone)]
struct AppData {
    client: reqwest::Client,
    oauth: OAuth,
    jwt_keys: (jsonwebtoken::EncodingKey, jsonwebtoken::DecodingKey),
    db: sea_orm::DatabaseConnection,
    session_store: RedisSessionStore,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // if this fails (i.e. railway deployment, not local), meh
    let _ = dotenv::dotenv();

    let port = std::env::var("PORT")
        .unwrap_or(String::from("1451"))
        .parse::<u16>()
        .context("$PORT not valid u16 port")?;

    let database_url = std::env::var("DATABASE_URL").context("need env var DATABASE_URL")?;
    let db = sea_orm::Database::connect(&database_url)
        .await
        .with_context(|| format!("can't connect to db at {database_url}"))?;
    Migrator::up(&db, None).await.context("migrate db")?;

    let jwt_secret = std::env::var_os("JWT_SECRET")
        .unwrap_or(OsString::from("0f3c13e6a2fc1e6ed08ed391de5e89276f72bb3a"));

    let redis_url = std::env::var("REDIS_URL").context("need env var REDIS_URL")?;
    let session_store = RedisSessionStore::new_pooled(
        Config::from_url(&redis_url)
            .create_pool(Some(Runtime::Tokio1))
            .context("create redis pool")?,
    )
    .await
    .with_context(|| format!("connect to redis at {redis_url}"))?;

    let app_data = &*Box::leak::<'static>(Box::new(AppData {
        client: reqwest::Client::new(),
        oauth: OAuth {
            frontend_url: std::env::var("FRONTEND_URL").context("need env var FRONTEND_URL")?,
            google: GoogleOAuthConfig {
                client_id: std::env::var("GOOGLE_OAUTH_CLIENT_ID")
                    .context("need env var GOOGLE_OAUTH_CLIENT_ID")?,
                client_secret: std::env::var("GOOGLE_OAUTH_CLIENT_SECRET")
                    .context("need env var GOOGLE_OAUTH_CLIENT_SECRET")?,
            },
        },
        jwt_keys: (
            jsonwebtoken::EncodingKey::from_secret(jwt_secret.as_encoded_bytes()),
            jsonwebtoken::DecodingKey::from_secret(jwt_secret.as_encoded_bytes()),
        ),
        db,
        session_store: session_store.clone(),
    }));

    let server = {
        HttpServer::new(move || {
            let session_middle = SessionMiddleware::new(session_store.clone(), Key::generate());

            App::new()
                .app_data(app_data)
                .service(actix_web::web::scope("/oauth/start").service(oauth::start::goog))
                .service(
                    actix_web::web::scope("/oauth/cb")
                        .service(oauth::cb::goog)
                        .wrap(session_middle),
                )
        })
        .bind(("127.0.0.1", port))
        .with_context(|| format!("bind to port {port}"))
    }?;

    eprintln!("ok, alive on port {port}...");

    Ok(server.run().await?)
}
