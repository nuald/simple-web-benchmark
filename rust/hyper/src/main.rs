extern crate futures;
extern crate hyper;
extern crate regex;

#[macro_use] extern crate lazy_static;

use futures::future::FutureResult;
use hyper::server::{Http, Request, Response, Service};
use regex::Regex;

struct HelloWorld;

impl Service for HelloWorld {
    // boilerplate hooking up hyper's server types
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = FutureResult<Self::Response, Self::Error>;

    fn call(&self, req: Request) -> Self::Future {
        futures::future::ok(
            match req.path() {
                "/" => {
                    Response::new().with_body("Hello World!")
                },
                path => {
                    lazy_static! {
                        static ref GREETING_RE: Regex = Regex::new(r"^/greeting/([a-z]+)$").unwrap();
                    }
                    let cap = GREETING_RE.captures(path).unwrap();
                    Response::new().with_body(format!("Hello, {}", &cap[1]))
                }
            }
        )
    }
}

fn main() {
    let addr = "127.0.0.1:3000".parse().unwrap();
    let server = Http::new().bind(&addr, || Ok(HelloWorld)).unwrap();
    server.run().unwrap();
}
