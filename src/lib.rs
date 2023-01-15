mod easy_html;

mod tcp_server;

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
