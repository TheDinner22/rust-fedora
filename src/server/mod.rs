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
use crate::server::lazy_body::LazyBody;
use crate::server::query_string::QueryString;

pub mod lazy_body;
pub mod query_string;

pub async fn try_start(
    port: u16,
    router: router::FedoraRouter,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // the service function which will handle every request
    // it is defined inside of the trystart function so that
    // it can access the router
    async fn my_service(
        req: Request<hyper::body::Incoming>,
    ) -> anyhow::Result<Response<BoxBody<Bytes, hyper::Error>>> {
        let (parts, incoming_body) = req.into_parts();

        let path: &str = parts.uri.path();
        let method: Method = parts.method;
        let query_string_object: QueryString = QueryString::new(parts.uri.query().unwrap_or(""));
        let header: HeaderMap = parts.headers;
        let body: LazyBody = LazyBody::new(incoming_body);

        match (method, path) {
            // Serve some instructions at /
            (Method::GET, "/") => {
                mk_response(format!("Try POSTing data to /echo such as: `curl localhost:3000/echo -XPOST -d 'hello world'`"))
            },

            // Simply echo the body back to the client.
            (Method::POST, "/echo") => {
                let b = body.into_bytes().await?;
                mk_response(b)
            }

            // Convert to uppercase before sending back to client
            (Method::POST, "/echo/uppercase") => {
                let b = body.into_bytes().await?.to_ascii_uppercase();
                mk_response(b)
            }

            // Reverse the entire body before sending back to the client.
            //
            // Since we don't know the end yet, we can't simply stream
            // the chunks as they arrive as we did with the above uppercase endpoint.
            // So here we do `.await` on the future, waiting on concatenating the full body,
            // then afterwards the content can be reversed. Only then can we return a `Response`.
            (Method::POST, "/echo/reversed") => {
                let b = body.into_bytes().await?;

                let reversed_body = b.iter().rev().cloned().collect::<Vec<u8>>();
                mk_response(reversed_body)
            }

            // Return the 404 Not Found for other routes.
            _ => {
                let mut not_found = Response::new(empty());
                *not_found.status_mut() = StatusCode::NOT_FOUND;
                Ok(not_found)
            }

        }
    }

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
                .serve_connection(stream, service_fn(my_service))
                .await
            {
                println!("Error serving connection: {:?}", err);
            }
        });
    }
}

// TODO put these somewhere else, this file is for 
// server stuff, not response helper functions
// makes an empty body
fn empty() -> BoxBody<Bytes, hyper::Error> {
    Empty::<Bytes>::new()
        .map_err(|never| match never {})
        .boxed()
}

// create a body from anything that can be
// converted into bytes
fn full<T: Into<Bytes>>(chunk: T) -> BoxBody<Bytes, hyper::Error> {
    Full::new(chunk.into())
        .map_err(|never| match never {})
        .boxed()
}

fn mk_response<T: Into<Bytes>>(chunk: T) -> anyhow::Result<Response<BoxBody<Bytes, hyper::Error>>> {
   Ok(Response::builder().body(full(chunk)).unwrap())
}

