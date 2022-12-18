mod easy_html;

mod tcp_server {
    use std::{net::{TcpListener, TcpStream}, io::{Read, Write}};

    use crate::easy_html;

    /// attempt to bind a TcpListener to a port on 127.0.0.1
    /// 
    /// port is the port to bind to (such as 3000 or 8080)
    /// 
    /// # Examples
    /// 
    /// ```
    /// let listener = try_start(3000).unwrap();
    /// 
    /// for stream in listener.incoming { ... }
    /// ```
    /// 
    /// # panics
    /// 
    /// This function never panics.
    pub fn try_start(port: u16) -> std::io::Result<()>{
        let socket_str = String::from("127.0.01:") + &port.to_string();

        let listener = TcpListener::bind(socket_str)?;

        for stream in listener.incoming() {
            let stream = stream.expect("for the stream to be good... todo!!");

            handle_connections(stream);
        }

        Ok(())
    }

    fn handle_connections(mut stream: TcpStream) {
        const BUFFER_SIZE: usize = 1024; // if this = 1 we get a deadlock becuase .read always returns at least 1 (0 implies stream shutdown)

        // get bytes from the stream, read from stream until it is empty
        let mut bytes = Vec::new();
        loop {
            let mut buffer = [0; BUFFER_SIZE];
            let bytes_read = stream.read(&mut buffer).unwrap();

            bytes.write_all(&buffer[0..bytes_read]).unwrap();

            if bytes_read == BUFFER_SIZE {
                continue;
            }
            else {
                break;
            }
        }
        
        let req_str = String::from_utf8_lossy(&bytes);
        println!("{:#?}", req_str);

        let response = "HTTP/1.1 200 OK\r\n\r\n".as_bytes();

        stream.write(response).unwrap();
        stream.flush().unwrap();
    }

}

pub mod server {
    use super::easy_html;
    use super::tcp_server;

    pub struct Server;

    impl Server {
        pub fn start(port: u16) -> Self{
            tcp_server::try_start(port).unwrap();
            Server
        }

        pub fn add_route<F>(&mut self, _route: &str, _f: F)
        where
            F: Fn(easy_html::Request) -> easy_html::Response
        {
            // todo!()
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
