extern crate actix_web;
use actix_web::{server, App, HttpRequest, Responder};

const HELLO: &str = "Hello world!";

fn greet(req: &HttpRequest) -> impl Responder {
    format!("{}", HELLO)
}

fn main() {
    let app = 
    
    server::new(
        || {
            App::new()
                .resource("/", |r| r.f(greet))
        }
    )
        .bind("127.0.0.1:8000")
        .expect("Can not bind to port 8000")
        .run();
}