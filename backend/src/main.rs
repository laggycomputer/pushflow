use anyhow::Context;
use http_body_util::Full;
use hyper::body::Bytes;
use hyper::Response;
use hyper_util::rt::TokioIo;
use std::convert::Infallible;
use std::net::SocketAddr;
use hyper::service::service_fn;
use tokio::net::TcpListener;

#[derive(Clone)]
pub struct Executor;

impl<F> hyper::rt::Executor<F> for Executor
where F: Future + Send + 'static,
    F::Output: Send + 'static,
{
    fn execute(&self, fut: F) {
        tokio::spawn(fut);
    }
}

async fn hello(_: hyper::Request<hyper::body::Incoming>) -> Result<Response<Full<Bytes>>, Infallible> {
    Ok(Response::new(Full::new(Bytes::from("Hello, World!"))))
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let port = std::env::var("PORT")
        .unwrap_or(String::from("1451"))
        .parse::<u16>()
        .context("$PORT not valid u16 port")?;

    let listener = TcpListener::bind(SocketAddr::from((
        [127, 0, 0, 1],
        port,
    ))).await.with_context(|| format!("bind to port {port}"))?;

    eprintln!("ok, alive on {}...", listener.local_addr()?);

    loop {
        let (stream, _) = listener.accept().await.context("await new conn")?;
        let io = TokioIo::new(stream);

        tokio::task::spawn(async move {
             // Handle the connection from the client using HTTP/2 with an executor and pass any
            // HTTP requests received on that connection to the `hello` function
            if let Err(err) = hyper::server::conn::http1::Builder::new()
                .serve_connection(io, service_fn(hello))
                .await
            {
                eprintln!("Error serving connection: {}", err);
            }
        });
    }

    Ok(())
}
