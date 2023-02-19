// see here for furhtering tut https://hyper.rs/guides/1/server/echo/

// query string
// method - ez with req.method()
// path - ez with req.uri.path()
// headers - their should be no issues working with req.headers() as its just a HashMap
// body

use std::net::SocketAddr;

use hyper::body::Bytes;
use http_body_util::{combinators::BoxBody, BodyExt, Empty, Full};
use hyper::body::Frame;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{body::Body, Method, Request, Response, StatusCode};
use tokio::net::TcpListener;

use std::collections::HashMap;
use std::cell::RefCell;

fn handle_request() -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    todo!()
}

/// # HashMap containing query string object
/// in short, this struct wraps a &'str which represents 
/// an incoming request's query string object and parses that
/// &str into a HashMap for you.
///
/// ## examples
/// ```
/// // valid query_params are parsed into HashMap
/// let URI = "https://some_website/users.com?password=123&foo=bar";
/// let query_params = LazyQueryString::new(URI);
/// assert_eq!(2, query_params.query_params().len());
///
/// // invalid params are ignored TODO
///
/// ```
///
/// ## implementation
/// This struct is implemented with following idea:
/// the query string object should be exposed to all handler functions; however,
/// if a handler function did not use the query string object, the &str was parsed for no reason 
/// and a HashMap was allocated for no reason!
///
/// So, this struct will not parse the &str into a HashMap unless the query_params are accesed.
/// This way, handlers can always choose to use the query string object; at the same time, those
/// which do not use it do not lose time or memory on an unused HashMap
///
/// ## panic
///
/// this struct and its methods should never panic.
struct LazyQueryString<'req> {
    raw_params: &'req str,
    param_map: Option<HashMap<&'req str, &'req str>>
}

impl<'req> LazyQueryString<'req> {
    fn new(query_string: &'req str) -> Self {
        LazyQueryString { raw_params: query_string, param_map: None }
    }

    // todo this should not be &mut using some horrid trick
    fn parse_raw_params(&mut self) {
        let query_map = self.raw_params
            .split('&')
            .filter_map(|pair| pair.split_once('='))
            .collect();

        self.param_map = Some(query_map);
    }

    fn query_params(&mut self) -> &HashMap<&'req str, &'req str> {
        if self.param_map.is_none() { self.parse_raw_params() }

        self.param_map.as_ref().expect("the option will always be Some. If it were none parse_raw_params would have been called")
    }
}

/// This is our service handler. It receives a Request, routes on its
/// path, and returns a Future of a Response.
async fn echo(
    req: Request<hyper::body::Incoming>
) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    match (req.method(), req.uri().path()) {
        // Serve some instructions at /
        (&Method::GET, "/") => Ok(Response::new(full(
            "Try POSTing data to /echo such as: `curl localhost:3000/echo -XPOST -d 'hello world'`",
        ))),

        // Simply echo the body back to the client.
        (&Method::POST, "/echo") => Ok(Response::new(req.into_body().boxed())),

        // Convert to uppercase before sending back to client using a stream.
        (&Method::POST, "/echo/uppercase") => {
            let frame_stream = req.into_body().map_frame(|frame| {
                let frame = if let Ok(data) = frame.into_data() {
                    data.iter()
                        .map(|byte| byte.to_ascii_uppercase())
                        .collect::<Bytes>()
                } else {
                    Bytes::new()
                };

                Frame::data(frame)
            });

            Ok(Response::new(frame_stream.boxed()))
        }

        // Reverse the entire body before sending back to the client.
        //
        // Since we don't know the end yet, we can't simply stream
        // the chunks as they arrive as we did with the above uppercase endpoint.
        // So here we do `.await` on the future, waiting on concatenating the full body,
        // then afterwards the content can be reversed. Only then can we return a `Response`.
        (&Method::POST, "/echo/reversed") => {
            // To protect our server, reject requests with bodies larger than
            // 64kbs of data.
            let max = req.body().size_hint().upper().unwrap_or(u64::MAX);
            if max > 1024 * 64 {
                let mut resp = Response::new(full("Body too big"));
                *resp.status_mut() = hyper::StatusCode::PAYLOAD_TOO_LARGE;
                return Ok(resp);
            }

            let whole_body = req.collect().await?.to_bytes();

            let reversed_body = whole_body.iter().rev().cloned().collect::<Vec<u8>>();
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

fn empty() -> BoxBody<Bytes, hyper::Error> {
    Empty::<Bytes>::new()
        .map_err(|never| match never {})
        .boxed()
}

fn full<T: Into<Bytes>>(chunk: T) -> BoxBody<Bytes, hyper::Error> {
    Full::new(chunk.into())
        .map_err(|never| match never {})
        .boxed()
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
                .serve_connection(stream, service_fn(echo))
                .await
            {
                println!("Error serving connection: {:?}", err);
            }
        });
    }
}
