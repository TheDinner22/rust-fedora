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

    pub async fn handle_request<'a>(
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

