use std::{
    io::{self, BufReader, Read, Write},
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
/// and uses them to determine how it will handle the body
///
/// The two headers that this function uses to determine how it will handle the body
/// are the `Transfer-Encoding: Chunked` and the `Content-Length: {length}` headers.
/// If neither header is present, it is assumed that there is no body.
///
/// ## errors
///
/// this function will error if
///
/// -both `Transfer-Encoding: Chunked` and `Content-Length: {length}` are present in the header
/// -`Transfer-Encoding` header is present with a value other than `Chunked`
///
pub fn try_dyn_read(mut stream: &TcpStream) -> io::Result<Vec<u8>> {
    let buf_reader = BufReader::new(&mut stream);

    // loop until the whole header has been sent
    let header_str = loop {
        let bytes = buf_reader.buffer();

        let inc_data_str = {
            use io::Error;
            use io::ErrorKind::Other;
            use std::str;

            str::from_utf8(bytes).map_err(|e| Error::new(Other, e.to_string()))
        }?;

        let header_is_valid = inc_data_str.split_whitespace().any(|line| line == "");

        if header_is_valid {
            break inc_data_str;
        }
    };

    // then we get the body if any from headers and then we quesry the buffer one more time and
    // then we get the body and then bytes and then BOOM! idk abt chunk encoded tho

    todo!()
}

#[cfg(test)]
mod tests;
