// types which make working with html requests and responses bearable
pub struct Response {
    status_code: u16, // todo is u32 the right size??
    headers: String, // todo is it type String???
    body: String, // todo is it type String???
}

impl From<u16> for Response {
    fn from(status_code: u16) -> Self {
        Response { status_code, headers: "".to_string(), body: "".to_string() }
    }
}

pub struct Request<'a> {
    body: &'a str, // todo is it type String???
    headers: &'a str, // todo is it type String???
    query_string_object: &'a str, // todo is it type String???
    path: &'a str, // todo is it type String???
    method: &'a str,
    http_ver: &'a str // todo is it type String???
}

impl<'a> TryFrom<Vec<u8>> for Request<'a> {
    type Error;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        todo!()
    }
}
