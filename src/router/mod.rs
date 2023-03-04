use hyper::body::Frame;
use hyper::body::Bytes;
use hyper::{body::Body, Method, Response, StatusCode, HeaderMap};
use http_body_util::{combinators::BoxBody, BodyExt, Empty, Full};

use crate::server::query_string::QueryString;
use crate::server::lazy_body::LazyBody;

// so the router is a service
// and you want to implement it and or force the user to impl it?
// its a struct which implement's this trait
// https://docs.rs/hyper/1.0.0-rc.3/hyper/service/trait.Service.html#associatedtype.Response
// and maybe has some built-in file routing and or a method or macro to 
// make api routes

// goals for the router
// easy to enable fs routing
// easy to add custom (usually api) routes

/// This is our service handler. It receives a Request, routes on its
/// path, and returns a Future of a Response.
pub async fn handle_request(
    req: hyper::Request<hyper::body::Incoming>,
) -> anyhow::Result<Response<BoxBody<Bytes, hyper::Error>>> {
    let (parts, incoming_body) = req.into_parts();

    let path: &str = parts.uri.path();
    let method: Method = parts.method;
    let query_string_object: QueryString = QueryString::new(parts.uri.query().unwrap_or(""));
    let header: HeaderMap = parts.headers;
    let body: LazyBody = LazyBody::new(incoming_body);

    match (method, path) {
        // Serve some instructions at /
        (Method::GET, "/") => Ok(Response::new(full(
            "Try POSTing data to /echo such as: `curl localhost:3000/echo -XPOST -d 'hello world'`",
        ))),

        // Simply echo the body back to the client.
        (Method::POST, "/echo") => {
            let b = body.into_bytes().await?;
            Ok(
                Response::new(full(b))
            )
        },

        // Convert to uppercase before sending back to client
        (Method::POST, "/echo/uppercase") => {
            let b = body.into_bytes().await?.to_ascii_uppercase();

            Ok(Response::new(full(b)))
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
            Ok(Response::new(full(reversed_body)))
        }

        // Return the 404 Not Found for other routes.
        _ => {
            let mut not_found = Response::new(empty());
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}

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

