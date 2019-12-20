extern crate hyper;
extern crate libc;
extern crate regex;

#[macro_use]
extern crate lazy_static;

use hyper::service::{make_service_fn, service_fn};
use hyper::StatusCode;
use hyper::{Body, Request, Response, Server};
use regex::Regex;
use std::convert::Infallible;
use std::fs;

async fn hello_world(req: Request<Body>) -> Result<Response<Body>, hyper::http::Error> {
    match req.uri().path() {
        "/" => Ok(Response::new(Body::from("Hello World!"))),
        path => {
            lazy_static! {
                static ref GREETING_RE: Regex = Regex::new(r"^/greeting/([a-z]+)$").unwrap();
            }
            match GREETING_RE.captures(path) {
                Some(cap) => Ok(Response::new(Body::from(format!("Hello, {}", &cap[1])))),
                None => Response::builder()
                    .status(StatusCode::NOT_FOUND)
                    .body(Body::from("404 Not Found\n")),
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let pid = unsafe { libc::getpid() }.to_string();
    fs::write(".pid", &pid).expect("Unable to read file");
    println!("Master {} is running", pid);
    let addr = ([127, 0, 0, 1], 3000).into();
    let new_svc = make_service_fn(|_conn| async {
        // service_fn converts our function into a `Service`
        Ok::<_, Infallible>(service_fn(hello_world))
    });
    let server = Server::bind(&addr).serve(new_svc);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
