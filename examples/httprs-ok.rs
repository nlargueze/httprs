//! Simple OK server

use std::net::SocketAddr;

use httprs::server::HttpServer;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // NB: dependency on tokio framework
    let listener = TcpListener::bind(addr).await?;
    eprintln!("Listening on {addr}");

    // We start a loop to continuously accept incoming connections
    loop {
        let (stream, _peer_addr) = listener.accept().await?;

        // To handle multiple connections concurrently, a task is spawned
        // for each incoming connection in the tokio executor
        tokio::task::spawn(async move {
            let res = HttpServer::new()
                .process_stream(stream, |_req| async {
                    //
                    Ok(())
                })
                .await;

            match res {
                Ok(_ok) => {
                    // OK
                }
                Err(err) => {
                    eprintln!("Error: {:?}", err);
                }
            };
        });
    }
}

// /// Handles a request
// async fn service_handler(req: ()) -> Result<(), Error> {
//     Ok(())
// }
