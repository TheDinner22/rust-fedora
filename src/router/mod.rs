use hyper::body::Frame;
use hyper::body::Bytes;
use hyper::{body::Body, Method, Response, StatusCode, HeaderMap};
use http_body_util::{combinators::BoxBody, BodyExt, Empty, Full};

use crate::server::query_string::QueryString;
use crate::server::lazy_body::LazyBody;

pub async fn handle_request<'req>(
    path: &str,
    method: Method,
    query_string_object: QueryString<'req>,
    header: HeaderMap,
    body: LazyBody
    ) -> anyhow::Result<Response<BoxBody<Bytes, hyper::Error>>>
{
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

