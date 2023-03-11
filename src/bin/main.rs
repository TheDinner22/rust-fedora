// rust-fedora is a library crate
// this binary is an example of how you could/should use the library

use rust_fedora::try_start;
use rust_fedora::FedoraRouter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let x = 123;
    try_start!(3000, x);

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
