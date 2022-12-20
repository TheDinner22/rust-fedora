mod easy_html;

mod tcp_server {
    use std::{net::{TcpListener, TcpStream}, io::{Read, Write, self, BufReader, BufRead}};

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

    /// # read a TcpStream and return its contents as a Vec<String>
    /// 
    /// This function uses a std::io::BufRead to read from a TcpStream.
    /// 
    /// ## what is different from std::io::BufRead.lines()?
    /// 
    /// 
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
    ///  Most likely, the TcpStream did not contain valid utf8 strings
    ///
    /// 
    /// 
    fn try_read_lines(mut stream: &TcpStream) -> io::Result<Vec<String>> {
        let buf_reader = BufReader::new(&mut stream);

        // read the stream and create an iterator that reads over it line by line
        // if every line is Ok, collect it into a Ok(Vec<String>)
        // if any of the lines has an error, return an io::Err
        // either way, return it
        buf_reader
            .lines()
            .take_while(|result| {
                !(result.as_ref().unwrap_or(&"some error".into()).is_empty()) // what the fuck
            })
            .collect::<io::Result<Vec<String>>>()
    }

    fn handle_connections(mut stream: TcpStream) {
        let http_request = try_read_lines(&stream).expect("http_request was invalid");

        println!("{:#?}", http_request);

        let response = "HTTP/1.1 200 OK\r\n\r\n".as_bytes();

        stream.write_all(response).unwrap();
    }

}

pub mod server {
    use super::easy_html;
    use super::tcp_server;

    pub struct Server;

    impl Server {
        pub fn start(port: u16) -> std::io::Result<Self>{
            tcp_server::try_start(port)?;
            Ok(Server)
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
