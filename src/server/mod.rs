// see here for furhtering tut https://hyper.rs/guides/1/server/echo/

// query string - made a struct for it
// method - ez with req.method()
// path - ez with req.uri.path()
// headers - their should be no issues working with req.headers() as its just a HashMap
// body

use std::net::SocketAddr;

use hyper::body::Bytes;
use http_body_util::{combinators::BoxBody, BodyExt, Empty, Full};
// for streaming the body use hyper::body::Frame;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{body::Body, Method, Request, Response, StatusCode, HeaderMap};
use tokio::net::TcpListener;

use crate::router;

pub mod query_string;
pub mod lazy_body;

/// This is our service handler. It receives a Request, routes on its
/// path, and returns a Future of a Response.
async fn fedora(
    req: Request<hyper::body::Incoming>
) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    let (parts, incoming_body) = req.into_parts();
    router::handle_request(parts.uri.path(), parts.method, query_string::QueryString::new(parts.uri.query().unwrap_or("")), parts.headers, lazy_body::LazyBody::new(incoming_body))
}


pub async fn try_start(port: u16) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = SocketAddr::from(([127, 0, 0, 1], port));

    // We create a TcpListener and bind it to 127.0.0.1:port
    let listener = TcpListener::bind(addr).await?;

    // We start a loop to continuously accept incoming connections
    loop {
        let (stream, _) = listener.accept().await?;

        // Spawn a tokio task to serve multiple connections concurrently
        tokio::task::spawn(async move {
            // Finally, we bind the incoming connection to our `hello` service
            if let Err(err) = http1::Builder::new()
                // `service_fn` converts our function in a `Service`
                .serve_connection(stream, service_fn(fedora))
                .await
            {
                println!("Error serving connection: {:?}", err);
            }
        });
    }
}
