// types which make working with html requests and responses bearable

pub struct Response {
    status_code: u32, // todo is u32 the right size??
    headers: String, // todo is it type String???
    body: String, // todo is it type String???
}

pub struct Request {
    body: String, // todo is it type String???
    headers: String, // todo is it type String???
    query_string_object: String, // todo is it type String???
    path: String, // todo is it type String???
    method: Method,

}

enum Method {
    GET,
    POST,
    PUT,
    DELETE
}