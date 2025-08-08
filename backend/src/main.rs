mod gated;
mod oauth;

use crate::oauth::{GoogleOAuthConfig, OAuth};
use actix_session::storage::RedisSessionStore;
use actix_session::SessionMiddleware;
use actix_web::cookie::Key;
use actix_web::{App, HttpServer, ResponseError};
use anyhow::Context;
use deadpool_redis::{Config, Runtime};
use migration::{Migrator, MigratorTrait};
use std::ffi::OsString;
use std::fmt::{Display, Formatter};

const FIXED_SESSION_KEY: [u8; 64] = [
    0xe9, 0xde, 0x52, 0x01, 0x07, 0xd0, 0xf9, 0x16,
    0xe3, 0x9a, 0x52, 0x39, 0x24, 0x68, 0xfd, 0xec,
    0x3f, 0xc2, 0x61, 0x74, 0xc4, 0xc5, 0x91, 0x1e,
    0xe6, 0x4d, 0x07, 0xa4, 0x07, 0x47, 0xdd, 0x90,
    0xec, 0x58, 0x29, 0x3a, 0x20, 0x56, 0xea, 0x1b,
    0x36, 0xb6, 0x97, 0xfd, 0xbe, 0x78, 0x6c, 0xd2,
    0x66, 0xf2, 0xbe, 0xc4, 0xcc, 0xbc, 0x5e, 0xb1,
    0x67, 0x11, 0x20, 0x56, 0xcf, 0x7d, 0xce, 0x26
];

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
            let session_middle = SessionMiddleware::new(
                session_store.clone(),
                match std::env::var("FREEZE_SESSION_KEY") {
                    Ok(_) => Key::from(&*FIXED_SESSION_KEY),
                    Err(_) => Key::generate()
                },
            );

            App::new()
                .app_data(app_data)
                .service(actix_web::web::scope("/oauth/start").service(oauth::start::goog))
                .service(
                    actix_web::web::scope("/oauth/cb")
                        .service(oauth::cb::goog)
                        .wrap(session_middle.clone()),
                )
                .service(
                    actix_web::web::scope("/gated")
                        .service(gated::check_auth)
                        .wrap(session_middle.clone()),
                )
        })
            .bind(("127.0.0.1", port))
            .with_context(|| format!("bind to port {port}"))
    }?;

    eprintln!("ok, alive on port {port}...");

    Ok(server.run().await?)
}
