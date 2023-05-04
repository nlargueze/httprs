//! Hyper 1.0 (RC3)
//!
//! # Results
//!
//! ## `curl localhost:3001 -v --http1.1`
//!
//! - sends: GET / HTTP/1.1
//! - receives: HTTP/1.1 200 OK
//!
//! ## `curl localhost:3001 -v --http2`
//!
//! - sends: GET / HTTP/1.1
//! - receives: HTTP/1.1 200 OK
//!
//! ## `curl localhost:3001 -v --http2-prior-knowledge`
//!
//! - sends: GET / HTTP/2
//! - receives (ERROR): HTTP/2 stream 1 was not closed cleanly before end of the underlying stream

use std::convert::Infallible;
use std::net::SocketAddr;

use http_body_util::Full;
use hyper::body::Bytes;
use hyper::server::conn::http1;
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
            if let Err(err) = http1::Builder::new()
                // `service_fn` converts our function in a `Service`
                .serve_connection(stream, service_fn(hello))
                .await
            {
                println!("Error serving connection: {:?}", err);
            }
        });
    }
}
