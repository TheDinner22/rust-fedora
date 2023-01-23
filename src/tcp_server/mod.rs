use std::{
    io::{self, BufRead, BufReader},
    net::{TcpListener, TcpStream},
};

// use crate::easy_html;

pub fn try_start(port: u16) -> std::io::Result<TcpListener> {
    let socket_str = String::from("127.0.0.1:") + &port.to_string();

    TcpListener::bind(socket_str)
}

/// # try and read an http request from a tcp stream
///
/// This function returns a result as the tcp stream could contain
/// anything including an invalid http request
///
/// # panics
///
/// this function never panics
///
/// # implementation
///
/// this function reads all of the headers from the incoming http request
/// the rest of the request is ignored. That is, this functions *gets* the headers
/// but does not parse, interpret, or validate the headers.
///
/// Neither does this function even attempt to read the body -that would require parsing the
/// content length or Transfer-Encoding!
///
/// ## errors
///
/// this function will error if
///
/// -there is an error reading the headers to utf8 string (if this happens, it is most likely that
/// there were invalid bytes in the request)
///
/// # returns
///
/// this function returns a RawHttp struct. This struct contains the raw, unparsed headers and the
/// buf_reader which, assuming the request is valid, contains the body, if any.
pub fn try_dyn_read(stream: &mut TcpStream) -> io::Result<RawHttp> {
    let mut buf_reader = BufReader::new(stream);

    // todo make this shit readable with a scope 4 definition
    let headers = (&mut buf_reader)
        .lines()
        .take_while(|line| !line.as_ref().unwrap_or(&String::new()).is_empty()) // error and empty string are both causes to stop so an error unwraps to ""
        .collect::<io::Result<Vec<String>>>()?;

    Ok(RawHttp {
        raw_headers: headers,
        body_reader: buf_reader,
    })
}

pub struct RawHttp<'stream> {
    raw_headers: Vec<String>,
    body_reader: BufReader<&'stream mut TcpStream>,
}

impl<'stream> RawHttp<'stream> {
    pub fn raw_headers(&self) -> &Vec<String> {
        &self.raw_headers
    }
}

#[cfg(test)]
mod tests;
