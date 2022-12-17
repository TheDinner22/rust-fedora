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

#[derive(Debug)]
pub struct Request<'a> {
    body: &'a str, // todo is it type String???
    headers: &'a str, // todo is it type String???
    query_string_object: &'a str, // todo is it type String???
    path: &'a str, // todo is it type String???
    method: &'a str,
    http_ver: &'a str // todo is it type String???
}

impl<'a> TryFrom<&'a str> for Request<'a> {
    type Error = String;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let mut crlf_split: Vec<&str> = value.split("\r\n").collect();
        let crlf_len = crlf_split.len();

        if crlf_len > 3 || crlf_len == 0 { return Err(format!("length was biger than 3 or it was 0! It was {}", crlf_len)); }

        else if crlf_len < 3 {
            let add_num = 3 - crlf_len;

            for _ in 0..add_num { crlf_split.push("") }
        }

        // get the Method Request-URI HTTP-Version
        let req_info_split: Vec<&str> = crlf_split[0].split_whitespace().collect();
        println!("{}", req_info_split.len()); //todo!!

        let method = req_info_split[0];
        let Uri = req_info_split[1];
        let http_ver = req_info_split[2];

        // get the headers
        let headers = crlf_split[1];

        // get the body
        let body = crlf_split[2];

        Ok(
            Request::<'a> {
                body,
                headers,
                query_string_object: "",
                path: Uri,
                method,
                http_ver
            }
        )
    }
}
