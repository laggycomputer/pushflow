use actix_web::web::Path;
use actix_web::{get, App, HttpServer, Responder};
use anyhow::Context;

#[get("/hello/{name}")]
async fn greet(name: Path<String>) -> impl Responder {
    format!("Hello {}!", name)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let port = std::env::var("PORT")
        .unwrap_or(String::from("1451"))
        .parse::<u16>()
        .context("$PORT not valid u16 port")?;

    let server = HttpServer::new(|| App::new().service(greet))
        .bind(("127.0.0.1", port))
        .with_context(|| format!("bind to port {port}"))?;

    eprintln!("ok, alive on port {port}...");

    Ok(server.run().await?)
}
