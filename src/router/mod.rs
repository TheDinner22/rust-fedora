use anyhow::Ok;
use http_body_util::{combinators::BoxBody, BodyExt, Empty, Full};
use hyper::body::Bytes;
use hyper::Request;
// use hyper::body::Frame;
use hyper::{body::Body, HeaderMap, Method, Response, StatusCode};

use crate::server::lazy_body::LazyBody;
use crate::server::query_string::QueryString;

// so the router is a service
// and you want to implement it and or force the user to impl it?
// its a struct which implement's this trait
// https://docs.rs/hyper/1.0.0-rc.3/hyper/service/trait.Service.html#associatedtype.Response
// and maybe has some built-in file routing and or a method or macro to
// make api routes

// goals for the router
// easy to enable fs routing
// easy to add custom (usually api) routes

// std::pin::Pin<Box<dyn std::future::Future<Output = Result<Response<Self::Res>, Self::Err>>>>
// is really fucked up way to say:
// async FnOnce closure that returns a Future which will resolve to
// A result.
// The Ok varient is an http response with a body that has type Res
// The Err varient is some error with type error

pub struct FedoraRouter {
    counter: i32
}

impl FedoraRouter {
    pub fn new() -> Self {
        FedoraRouter { counter: 0 }
    }

    fn add_route(&mut self) -> anyhow::Result<()> {
        todo!()
    }

    fn add_file_route(&mut self) -> anyhow::Result<()> {
        todo!()
    }

    async fn handle_request<'a>(
        &self,
        path: &str,
        method: Method,
        query_string_object: QueryString<'a>,
        headers: HeaderMap,
        body: LazyBody,
    ) -> Result<Response<BoxBody<Bytes, hyper::Error>>, anyhow::Error> {
        todo!()
    }
}

impl hyper::service::Service<Request<hyper::body::Incoming>> for FedoraRouter {
    type Response = Response<BoxBody<Bytes, hyper::Error>>;
    type Error = anyhow::Error;
    type Future = std::pin::Pin<Box<dyn std::future::Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn call(&mut self, req: Request<hyper::body::Incoming>) -> Self::Future {
        let (parts, incoming_body) = req.into_parts();

        let path: &str = parts.uri.path();
        let method: Method = parts.method;
        let query_string_object: QueryString = QueryString::new(parts.uri.query().unwrap_or(""));
        let header: HeaderMap = parts.headers;
        let body: LazyBody = LazyBody::new(incoming_body);

        let res = match (method, path) {
            // Serve some instructions at /
            (Method::GET, "/") => {
                self.counter += 1;
                mk_response(format!("Try POSTing data to /echo such as: `curl localhost:3000/echo -XPOST -d 'hello world {}'`", self.counter))
            },

            _ => {
                todo!()
            }

            /*
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
            */

        };

        Box::pin( async { res })
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

fn mk_response<T: Into<Bytes>>(chunk: T) -> anyhow::Result<Response<BoxBody<Bytes, hyper::Error>>> {
   Ok(Response::builder().body(full(chunk)).unwrap())
}
