use std::{collections::HashMap, io::Write, net::TcpStream};

pub struct Response {
    status_code: u16,
    reason_phrase: String,

    headers: HashMap<String, String>,
    body: Vec<u8>,
    // http_ver: u8, commented because this really only sends http/1.1
}

impl Response {
    fn reason_phrase(status_code: u16) -> String {
        match status_code {
            200 => "OK".into(),
            400 => "Bad Request".into(),
            401 => "Unauthorized".into(),
            403 => "Forbidden".into(),
            404 => "Not Found".into(),
            500 => "Internal Server Error".into(),
            _ => "No Reason Phrase".into(),
        }
    }

    fn into_bytes(self) {
        // things to construct a status line
        const HTTP_VER: &str = "HTTP/1.1 200 OK";
        let reason_phrase = Response::reason_phrase(self.status_code);
        let status_line = format!("{} {} {}", HTTP_VER, self.status_code, reason_phrase);

        let headers_string = self.headers
            .into_iter()
            .map(|(key, value)| )
    }

    fn send_down_stream(self, mut stream: TcpStream) -> std::io::Result<()> {
        // first convert the Response to bytes
        let bytes = self.into_bytes();

        // write those bytes to the stream
        stream.write_all(bytes)
    }
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
