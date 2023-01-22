pub struct Response {
    status_code: u16, // todo is u32 the right size??
    headers: String,  // todo is it type String???
    body: String,     // todo is it type String???
}

impl From<u16> for Response {
    fn from(status_code: u16) -> Self {
        Response {
            status_code,
            headers: "".to_string(),
            body: "".to_string(),
        }
    }
}
