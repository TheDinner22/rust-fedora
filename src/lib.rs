mod tcp_server {
    use std::{net::TcpListener, vec};

    
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
    fn try_start(port: u16) -> std::io::Result<TcpListener>{
        let socket_str = String::from("127.0.01:") + &port.to_string();

        TcpListener::bind(socket_str)
    }

    fn handle_connections(listener: TcpListener) {
        for stream in listener.incoming() {
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
