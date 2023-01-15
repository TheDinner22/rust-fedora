mod easy_html;

mod tcp_server {
    use std::{
        io::{self, Read, Write},
        net::{TcpListener, TcpStream},
    };

    // use crate::easy_html;

    pub fn try_start(port: u16) -> std::io::Result<TcpListener> {
        let socket_str = String::from("127.0.0.1:") + &port.to_string();

        TcpListener::bind(socket_str)
    }

    /// # read a TcpStream and return its contents as a Vec<String>
    ///
    /// This function uses a std::io::BufRead to read from a TcpStream.
    ///
    /// ## what is different from std::io::BufRead.lines()?
    ///
    /// There is some internal logic which stops reading from the stream
    /// once the http request has been fully recived.
    ///
    /// In short, calling the `lines` method will continue reading until
    /// the connection closes (or its told to stop reading). This function
    /// handles telling the `lines` method to stop reading.
    ///
    /// ## panics
    ///
    /// This function should never panic
    ///
    /// ## errors
    ///
    /// This function could return a std::io::Error.
    /// That means that the error occured when reading the contents of the stream.
    ///
    ///  Most likely, the TcpStream did not contain all valid utf8 strings
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
}

pub mod server {
    use std::io::{self, Write};
    use std::net::{TcpListener, TcpStream};

    use super::easy_html;
    use super::tcp_server;

    pub struct Server {
        listener: TcpListener,
    }

    impl Server {
        pub fn try_start(port: u16) -> std::io::Result<Self> {
            let listener = tcp_server::try_start(port)?;

            let server = Server { listener };

            for stream in server.listener.incoming() {
                let stream = stream.expect("stream was not valid!");

                server.handle_connection(stream)?;
            }

            Ok(server)
        }

        fn handle_connection(&self, mut stream: TcpStream) -> io::Result<()> {
            let http_request = tcp_server::try_dyn_read(&stream)?;

            let request = {
                use io::{Error, ErrorKind::Other};
                easy_html::Request::try_from(&http_request).map_err(|e| Error::new(Other, e))
            }?;

            println!("{:#?}", request);

            let response = "HTTP/1.1 200 OK\r\n\r\n".as_bytes();

            stream.write_all(response)?;

            Ok(())
        }

        pub fn add_route<F>(&mut self, _route: &str, _f: F)
        where
            F: Fn(easy_html::Request) -> easy_html::Response,
        {
            todo!()
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
