// rust-fedora is a library crate
// this binary is an example of how you could/should use the library

use rust_fedora::server;
use rust_fedora::FedoraRouter;


#[tokio::main]
async fn main(){
    let _my_server = server::try_start(3000, FedoraRouter::new()).await.unwrap();

    // my_server.add_route("/", |data| {
    //     return 200.into(); 
    // })
}

/*
todo for project:
- ways to add routes
- routing in general
- config vars (macros would make this awesome)
- ez logs
- https?
- better front end api
- public folder
- html or the other thing??? both??
- 
*/
