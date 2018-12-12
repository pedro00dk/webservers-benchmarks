extern crate hyper;

use hyper::{Body, Request, Response, Server};
use hyper::rt::Future;
use hyper::service::service_fn_ok;


const HELLO: &str = "Hello world!";


fn hello_world(_req: Request<Body>) -> Response<Body> {
    Response::new(Body::from(HELLO))
}

fn main() {
    let server = Server::bind(&([127, 0, 0, 1], 8000).into())
        .serve(|| service_fn_ok(hello_world))
        .map_err(|e| eprintln!("server error: {}", e));
    hyper::rt::run(server);
}