
extern crate iron;
extern crate router;

use std::str::FromStr;
use std::net;
use iron::{Iron, Request, Response, IronResult};
use router::Router;
use iron::status;

fn hello(_: &mut Request) -> IronResult<Response> {
    let response = Response::with((status::Ok, "Hello World!"));
    Ok(response)
}

fn get_server_port() -> u16 {
    3000
}

fn main() {
    let mut router = Router::new();
    router.get("/", hello);

    Iron::new(router).http("localhost:3000").unwrap();
}
