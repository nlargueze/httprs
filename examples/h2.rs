//! H2 example
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

use http::{Response, StatusCode};
use tokio::net::TcpListener;

#[tokio::main]
pub async fn main() {
    let listener = TcpListener::bind("127.0.0.1:3001").await.unwrap();

    // Accept all incoming TCP connections.
    loop {
        if let Ok((socket, _peer_addr)) = listener.accept().await {
            // Spawn a new task to process each connection.
            tokio::spawn(async {
                // Start the HTTP/2 connection handshake
                let mut h2_conn = h2::server::handshake(socket).await.unwrap();
                // Accept all inbound HTTP/2 streams sent over the
                // connection.
                while let Some(request) = h2_conn.accept().await {
                    let (request, mut respond) = request.unwrap();
                    println!("Received request: {:?}", request);

                    // Build a response with no body
                    let response = Response::builder().status(StatusCode::OK).body(()).unwrap();

                    // Send the response back to the client
                    respond.send_response(response, true).unwrap();
                }
            });
        }
    }
}
