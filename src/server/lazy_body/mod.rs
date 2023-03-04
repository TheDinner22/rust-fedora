use anyhow::{bail, Ok};
use http_body_util::BodyExt;

// code for me for later
//
// To protect our server, reject requests with bodies larger than
// 64kbs of data.
// let max = req.body().size_hint().upper().unwrap_or(u64::MAX);
// if max > 1024 * 64 {
//     let mut resp = Response::new(full("Body too big"));
//     *resp.status_mut() = hyper::StatusCode::PAYLOAD_TOO_LARGE;
//     return Ok(resp);
// }

/// # a wrapper for an incoming http request body
///
/// You cannot stream the body with this struct, only await the entire body
/// and collect it as Bytes
///
/// TODO this needs to do a check to make sure the body is not too big
///
/// ## async await
///
/// The first call to either the get_entire_body method or the into_bytes method calls .await on the incoming body.
/// Under the hood, the first call to either of those methods strores the body in the struct.
/// Subsequent calls to get_entire_body or into_bytes just get the body from a private field on the struct.
///
/// That is, if you call get_entire_body and await the future it returns. Every other future returned 
/// by this struct's methods will resolve instantly.
///
/// Another way to phrase this is that the body is lazy-loaded on the first access.
///
/// So the futures returned by the 2nd, 3rd, etc. calls to get_entire_body all resolve instantly!
///
/// ## errors
///
/// I am pretty sure that if any of the methods return an error, the body
/// is lost and there is no way to recover any part of it.
pub struct LazyBody {
    incoming: Option<hyper::body::Incoming>,
    raw_body: Option<hyper::body::Bytes>
}

impl LazyBody {
    pub fn new(incoming_body: hyper::body::Incoming) -> Self {
        LazyBody { incoming: Some(incoming_body), raw_body: None }
    }

    async fn await_body(&mut self) -> anyhow::Result<()> {
        // if the inc stream has yet to be parsed, parse it!
        if let Some(incoming_strem) = self.incoming.take() {
            self.raw_body = Some(incoming_strem.collect().await?.to_bytes());
        }

        Ok(())
    }

    pub async fn get_entire_body(&mut self) -> anyhow::Result<&hyper::body::Bytes> {
        // parse the body if we have not already
        self.await_body().await?;

        // if there is a body, return it!
        if let Some(bytes) = self.raw_body.as_ref() {
            return Ok(bytes);
        }

        // otherwise error
        bail!("body parsing failed and body unrecoverable");
    }

    /// # convert this struct into Bytes
    ///
    /// ## errors
    ///
    /// if there was an error parsing the body, during this method call or in a 
    /// previous method call, this function will error.
    pub async fn into_bytes(mut self) -> anyhow::Result<hyper::body::Bytes> {
        // parse the body if we have not already
        self.await_body().await?;

        // if there is a body, return it!
        if let Some(bytes) = self.raw_body {
            return Ok(bytes);
        }

        // otherwise error
        bail!("body parsing failed and body unrecoverable");
    }
}

