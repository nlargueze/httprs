# httprs

HTTP server and client

WORK IN PROGRESS

## Dev notes

### TCP layer

It is possible to listen concurrently to multiple TCP connections/ports in `tokio` with:

```rs
tokio::join!(
    warp::serve(routes).run(([127, 0, 0, 1], 3030)),
    warp::serve(routes).run(([127, 0, 0, 1], 3031)),
);
```

Or streams can be combined as:

```rs
use std::net::Ipv4Addr;
use tokio::net::TcpListener;
use tokio_stream::{StreamExt, wrappers::TcpListenerStream};

let listener1 = TcpListener::bind((Ipv4Addr::LOCALHOST, 3030)).await?;
let listener2 = TcpListener::bind((Ipv4Addr::LOCALHOST, 3031)).await?;

let stream1 = TcpListenerStream::new(listener1);
let stream2 = TcpListenerStream::new(listener2);

let combined = stream1.merge(stream2);

warp::serve(routes).run_incoming(combined).await?;
```

### TLS

- [tokio-tls](https://docs.rs/tokio-tls/0.3.1/tokio_tls/) is the crate for async TLS streams with Tokio

### HTTP/1

- [httparse](https://docs.rs/httparse/latest/httparse) is the crate used to parse HTTP/1 streams to request
- [http](https://docs.rs/http/latest/http/) is the crate defining Requests and Responses

### HTTP/2

- [Specs](https://httpwg.org/specs/rfc7540.html#starting)
- [h2](https://docs.rs/h2/latest/h2) provides Rust support for HTTP/2.
- 3 ways to negotiate a HTTP/2 connection:
  - `upgrade`: send a HTTP/1 request with an `Upgrade` header ([specs](https://developer.mozilla.org/en-US/docs/Web/HTTP/Protocol_upgrade_mechanism))
  - `h2`: send a TLS request and use the `ALPN`mechanism
  - pre-agreed protocol between the client and server
- On the browser side, HTTP/2 requires TLS (see [why](https://stackoverflow.com/questions/46788904/why-do-web-browsers-not-support-h2c-http-2-without-tls)):
