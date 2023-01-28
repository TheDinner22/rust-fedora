use super::*;

fn make_empty_raw_http_req<'stream>() -> RawHttp<'stream> {
    RawHttp {
        raw_headers: vec![],
        body_reader: RefCell::new(None),
    }
}

#[test]
fn try_dyn_read_works() {
    // i need to be able to send reqests firstLL
    unimplemented!()
}

#[test]
fn raw_http_take_body_stream_never_panics() {
    let raw_http_request = &mut make_empty_raw_http_req();

    // calling the public getter methods should never panic
    let _headers_ref = raw_http_request.raw_headers();
    let _body = raw_http_request.take_body_stream(29);
    let _body = raw_http_request.take_body_stream(29);
    let _body = raw_http_request.take_body_stream(29);
    let _body = raw_http_request.take_body_stream(29);
    let _body = raw_http_request.take_body_stream(29);
    let _body = raw_http_request.take_body_stream(29);
}

#[test]
#[should_panic]
fn raw_http_take_body_stream_panics_when_using_private_fields() {
    let raw_http_request = &mut make_empty_raw_http_req();

    let _private_field = raw_http_request.body_reader.borrow();

    // calling the public getter methods should never panic
    let _headers_ref = raw_http_request.raw_headers();
    let _body = raw_http_request.take_body_stream(29);
    let _body = raw_http_request.take_body_stream(29);
    let _body = raw_http_request.take_body_stream(29);
    let _body = raw_http_request.take_body_stream(29);
    let _body = raw_http_request.take_body_stream(29);
    let _body = raw_http_request.take_body_stream(29);
}
