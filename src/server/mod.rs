//! HTTP server

use std::future::Future;

use tokio::net::TcpStream;

use self::error::Error;

pub mod error;

/// HTTP server
#[derive(Debug, Default)]
pub struct HttpServer {}

impl HttpServer {
    /// Instantiates a HTTP server
    pub fn new() -> Self {
        Self {}
    }

    /// Processes a received TCP stream
    pub async fn process_stream<F, Fut>(&self, _stream: TcpStream, _f: F) -> Result<(), Error>
    where
        F: Fn(()) -> Fut,
        Fut: Future<Output = Result<(), Error>>,
    {
        // Consume the stream
        // Pass the HTTP_handler
        // Return the response as a stream
        todo!()
    }
}
