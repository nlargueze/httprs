//! Hyper 1.0 (RC3) - HTTP/2
//!
//! # Results
//!
//! ## `curl localhost:3001 -v --http1.1`
//!
//! - sends: GET / HTTP/1.1
//! - receives (ERROR): Received HTTP/0.9 when not allowed
//!
//! ## `curl localhost:3001 -v --http2`
//!
//! - sends: GET / HTTP/1.1
//! - receives (ERROR): Received HTTP/0.9 when not allowed
//!
//! ## `curl localhost:3001 -v --http2-prior-knowledge`
//!
//! - sends: GET / HTTP/2
//! - receives: HTTP/2 200

use std::convert::Infallible;
use std::future::Future;
use std::net::SocketAddr;

use http_body_util::Full;
use hyper::body::Bytes;
use hyper::rt::Executor;
use hyper::server::conn::http2;
use hyper::service::service_fn;
use hyper::{Request, Response};
use tokio::net::TcpListener;

async fn hello(_: Request<hyper::body::Incoming>) -> Result<Response<Full<Bytes>>, Infallible> {
    Ok(Response::new(Full::new(Bytes::from("Hello, World!"))))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));

    // We create a TcpListener and bind it to 127.0.0.1:3000
    let listener = TcpListener::bind(addr).await?;

    // We start a loop to continuously accept incoming connections
    loop {
        let (stream, _) = listener.accept().await?;

        // Spawn a tokio task to serve multiple connections concurrently
        tokio::task::spawn(async move {
            // Finally, we bind the incoming connection to our `hello` service
            if let Err(err) = http2::Builder::new(TokioExecutor)
                // `service_fn` converts our function in a `Service`
                .serve_connection(stream, service_fn(hello))
                .await
            {
                println!("Error serving connection: {:?}", err);
            }
        });
    }
}

#[derive(Clone)]
struct TokioExecutor;

impl<F> Executor<F> for TokioExecutor
where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
{
    fn execute(&self, future: F) {
        tokio::spawn(future);
    }
}
