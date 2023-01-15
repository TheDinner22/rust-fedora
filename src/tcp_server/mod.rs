use std::{
    io::{self, Read, Write},
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
///
/// ## errors
///
/// this function will error if
///
/// -both `Transfer-Encoding: Chunked` and `Content-Length: {length}` are present in the header
/// -`Transfer-Encoding` header is present with a value other than `Chunked`
///
/// - both
pub fn try_dyn_read(mut stream: &TcpStream) -> io::Result<Vec<u8>> {
    const BUFFER_SIZE: usize = 1024;

    let mut bytes = Vec::new();
    loop {
        let mut buffer = [0; BUFFER_SIZE];
        let bytes_read = stream.read(&mut buffer)?;

        bytes.write_all(&buffer[0..bytes_read])?;

        if bytes_read == BUFFER_SIZE {
            continue;
        } else {
            break Ok(bytes);
        }
    }
}

#[cfg(test)]
mod tests;
