// see here for furhtering tut https://hyper.rs/guides/1/server/echo/
// this ends up being boilerplate for working with hyper

use std::net::SocketAddr;

use http_body_util::{combinators::BoxBody, BodyExt, Empty, Full};
use hyper::body::Bytes;
// for streaming the body use hyper::body::Frame;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{body::Body, HeaderMap, Method, Request, Response, StatusCode};
use tokio::net::TcpListener;

use crate::router;
use crate::svc;

pub mod lazy_body;
pub mod query_string;

pub async fn try_start(
    port: u16,
    //router: router::FedoraRouter,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
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
                .serve_connection(stream, svc::Svc::new() )
                .await
            {
                println!("Error serving connection: {:?}", err);
            }
        });
    }
}
