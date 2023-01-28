use std::{collections::HashMap, io::Write, net::TcpStream};

pub struct Response {
    status_code: u16,

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

    // # convert self into an http request
    //
    // this function consumes the respose and creates a Vec<u8>
    // representing an http request as bytes.
    //
    // You might use this function to convert the http response to bytes so that you can send it
    // through a TcpStream
    //
    // # checks
    //
    // this function makes no checks to see if self contains data that can be parsed into
    // a valid http request. It is very possible that an instance of Response containing invalid
    // data to create an invalid http response.
    //
    // Examples of invalid data include inncorrectly formatted headers or a status code greater
    // than 599
    fn into_bytes(self) -> Vec<u8> {
        // things to construct a status line
        const HTTP_VER: &str = "HTTP/1.1 200 OK";
        let reason_phrase = Response::reason_phrase(self.status_code);

        let status_line = format!("{} {} {}\r\n", HTTP_VER, self.status_code, reason_phrase);

        let headers_string = self
            .headers
            .into_iter()
            .fold(String::new(), |accum, (key, value)| {
                accum + &key + ": " + &value + "\r\n"
            });

        // now convert the status_line, headers, and body into iterators over bytes
        // then chain them together to get the byte-version of the request
        let request_as_bytes: Vec<u8> = {
            status_line
                .bytes() // the reason_phrase
                .chain(headers_string.bytes()) // plus the headers
                .chain("\r\n".bytes()) // CRLF to indicate end of headers
                .chain(self.body.into_iter()) // plus the body
                .collect()
        };

        request_as_bytes
    }

    // # send this request down a TcpStream
    //
    // this fucntion converts the Response to bytes
    // and the calls stream.write_all(bytes)
    //
    // # blocking
    //
    // that is, this stream will block until all of the data has been sent or
    // until there is an error
    //
    // # errors
    //
    // this function will only error if there is an issue writing to the stream. It forwards the
    // io::Result returned by the stream.write_all method.
    //
    // # checks
    //
    // this function does not check anything about the request! That means passing an unchecked
    // Response to this function could lead to an invalid http response being sent. (for example,
    // the Content-Length header may be unset or inaccurate)
    //
    pub fn send_down_stream(self, mut stream: TcpStream) -> std::io::Result<()> {
        // first convert the Response to bytes
        let bytes = self.into_bytes();

        // write those bytes to the stream
        stream.write_all(&bytes)
    }
}

impl From<u16> for Response {
    fn from(status_code: u16) -> Self {
        Response {
            status_code,
            reason_phrase: Response::reason_phrase(status_code),
            headers: HashMap::new(),
            body: Vec::new(),
        }
    }
}
