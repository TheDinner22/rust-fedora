use std::{
    cell::RefCell,
    io::{self, BufRead, BufReader, Read},
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
///
/// # blocking
///
/// if a tcp request is sent such that it does parse nicely into a Vec<String> (newline seperated), but none of those
/// strings are empty, this function will block for 5-10 minutes or until the tcp connection is
/// closed.
pub fn try_dyn_read(stream: &mut TcpStream) -> io::Result<RawHttp> {
    let mut buf_reader = BufReader::new(stream);

    // todo make this shit readable with a scope 4 definition
    let headers = (&mut buf_reader)
        .lines()
        .take_while(|line| !line.as_ref().unwrap_or(&String::new()).is_empty()) // error and empty string are both causes to stop so an error unwraps to ""
        .collect::<io::Result<Vec<String>>>()?;

    Ok(RawHttp {
        raw_headers: headers,
        body_reader: RefCell::new(Some(buf_reader)),
    })
}

pub struct RawHttp<'stream> {
    raw_headers: Vec<String>,
    body_reader: RefCell<Option<BufReader<&'stream mut TcpStream>>>,
}

impl<'stream> RawHttp<'stream> {
    pub fn raw_headers(&self) -> &Vec<String> {
        &self.raw_headers
    }

    /// # take from a buf reader
    ///
    /// This function reads from a tcp stream wrapped in a buf_reader.
    /// It calls .bytes() on the buf reader and then calls .take on that byte iter.
    ///
    /// # RefCell and interior mutablility
    ///
    /// this function should really only be called once! The BufReader is wrapped in an option which
    /// is then wrapped in a refcell. In short, even though this function only takes a &self, self
    /// is mutated when this function is called.
    ///
    /// The reason this function has to do that is that it is reading from a buf_reader. The way the
    /// function does that requires consuming the buf_reader which requires mutating self! This is a
    /// side affect of the fact that reading from a stream or buf_reader requires more than a read
    /// only reference (because the data is being moved out of some internal buffer).
    ///
    /// # panic
    ///
    /// When used without access to the private fields of this struct, this function should never
    /// panic!
    ///
    /// This function calls `let body_reader = self.body_reader.borrow_mut().take();`
    /// as you can see, the mutable borrow is consumed in the same line it is created.
    ///
    /// That is, when accessed from outside its module, the only way to access the RefCell is through this this method which *only borrows
    /// the inner option as mutable once*. Thus, creating multiple mutable references at the same
    /// time is impossible.
    ///
    /// To anyone who is writing code which CAN access the private fields in this struct (maybe
    /// future me who is writing a unit test) ***DO NOT ACCESS THE FIELDS***! Borrowing or mutably
    /// borrowing from the RefCell and then calling this function will causes a runtime panic.
    ///
    /// # TLDR
    ///
    /// only use the getter methods defined on this struct: they are safe. Never access the struct
    /// fields directly
    pub fn take_body_stream(&self, length: usize) -> std::io::Result<Vec<u8>> {
        let body_reader = self.body_reader.borrow_mut().take();

        if body_reader.is_none() {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "error: already consumed the requests body",
            ));
        }

        // todo this might end up half reading a body if the request is closed prematurely!
        body_reader.unwrap().bytes().take(length).collect()
    }
}

#[cfg(test)]
mod tests;
