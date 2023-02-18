// rust-fedora is a library crate
// this binary is an example of how you could/should use the library

use rust_fedora::server;

fn main(){
    let _my_server = server::try_start(3000).unwrap();

    // my_server.add_route("/", |data| {
    //     return 200.into(); 
    // })
}

/*
todo for project:
- better types (implement to and from everywhere)
- helper functions in a helper module
- lots of tests
- other ways to add routes
- config vars
- ez logs
- https
- better front end api
- public folder
- html or the other thing??? both??
- 
*/
