extern crate hyper;
extern crate libc;
extern crate regex;

#[macro_use] extern crate lazy_static;

use hyper::StatusCode;
use hyper::{Body, Request, Response, Server};
use hyper::rt::Future;
use hyper::service::service_fn_ok;
use regex::Regex;
use std::fs;

fn hello_world(req: Request<Body>) -> Response<Body> {
    match req.uri().path() {
        "/" => {
            Response::new(Body::from("Hello World!"))
        },
        path => {
            lazy_static! {
                static ref GREETING_RE: Regex = Regex::new(r"^/greeting/([a-z]+)$").unwrap();
            }
            match GREETING_RE.captures(path) {
                Some(cap) => Response::new(Body::from(format!("Hello, {}", &cap[1]))),
                None => Response::builder()
                     .status(StatusCode::NOT_FOUND)
                     .body(Body::from("404 Not Found\n"))
                     .unwrap()
            }
        }
    }
}

fn main() {
    let pid = unsafe { libc::getpid() }.to_string();
    fs::write(".pid", &pid).expect("Unable to read file");
    println!("Master {} is running", pid);
    let addr = ([127, 0, 0, 1], 3000).into();
    let new_svc = || {
        // service_fn_ok converts our function into a `Service`
        service_fn_ok(hello_world)
    };
    let server = Server::bind(&addr)
        .serve(new_svc)
        .map_err(|e| eprintln!("server error: {}", e));

    hyper::rt::run(server);
}
