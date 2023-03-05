use http_body_util::combinators::BoxBody;
use hyper::body::Bytes;
use http_body_util::{Full, BodyExt};
use hyper::server::conn::http1;
use hyper::service::Service;
use hyper::{body::Incoming as IncomingBody, Request, Response};
use tokio::net::TcpListener;

use std::future::Future;
use std::net::SocketAddr;
use std::pin::Pin;

type Counter = i32;

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
//     let addr: SocketAddr = ([127, 0, 0, 1], 3000).into();

//     let listener = TcpListener::bind(addr).await?;
//     println!("Listening on http://{}", addr);

//     loop {
//         let (stream, _) = listener.accept().await?;

//         tokio::task::spawn(async move {
//             if let Err(err) = http1::Builder::new()
//                 .serve_connection(stream, Svc { counter: 81818 })
//                 .await
//             {
//                 println!("Failed to serve connection: {:?}", err);
//             }
//         });
//     }
// }

pub struct Svc {
    pub counter: Counter,
    string: String // works with things that are not copy!
}

impl Svc {
    pub fn new() -> Self {
        Svc { counter: 123, string: String::from("dskfjaklfjdas;lfkj") }
    }
}

impl Service<Request<IncomingBody>> for Svc {
    type Response = Response<BoxBody<Bytes, hyper::Error>>;
    type Error = anyhow::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn call(&mut self, req: Request<IncomingBody>) -> Self::Future {
        fn full<T: Into<Bytes>>(chunk: T) -> BoxBody<Bytes, hyper::Error> {
            Full::new(chunk.into())
                .map_err(|never| match never {})
                .boxed()
        }

        fn mk_response(s: String) -> anyhow::Result<Response<BoxBody<Bytes, hyper::Error>>> {
           Ok(Response::builder().body(full(s)).unwrap())
        }

        let res = match req.uri().path() {
            "/" => mk_response(format!("home! counter = {:?}", self.counter)),
            "/posts" => mk_response(format!("posts, of course! counter = {:?}", self.counter)),
            "/authors" => mk_response(format!(
                "authors extraordinare! counter = {:?}",
                self.counter
            )),
            // Return the 404 Not Found for other routes, and don't increment counter.
            _ => return Box::pin(async { mk_response("oh no! not found".into()) }),
        };

        if req.uri().path() != "/favicon.ico" {
            self.counter += 1;
        }

        Box::pin(async { res })
    }
}

