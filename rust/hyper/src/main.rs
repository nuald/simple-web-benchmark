use getopts::Options;
use hyper::http::Error;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server, StatusCode};
use regex::Regex;
use std::convert::Infallible;
use std::{env, fs, process};

lazy_static::lazy_static! {
    static ref GREETING_RE: Regex = Regex::new(r"^/greeting/([a-z]+)$").unwrap();
}

async fn hello_world(req: Request<Body>) -> Result<Response<Body>, Error> {
    match req.uri().path() {
        "/" => Ok(Response::new(Body::from("Hello World!"))),
        path => match GREETING_RE.captures(path) {
            Some(cap) => Ok(Response::new(Body::from(format!("Hello, {}", &cap[1])))),
            None => Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::from("404 Not Found\n")),
        },
    }
}

#[tokio::main]
async fn main() {
    let pid = process::id().to_string();
    fs::write(".pid", &pid).expect("Unable to write file");

    let args = env::args().collect::<Vec<String>>();
    let mut opts = Options::new();
    opts.optopt("", "port", "server port", "PORT");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!("{}", f),
    };

    let port = matches.opt_get::<u16>("port").unwrap().unwrap_or(3000);
    println!("Master {} is running on port {}", pid, port);

    let addr = ([127, 0, 0, 1], port).into();
    let new_svc = make_service_fn(|_conn| async {
        // service_fn converts our function into a `Service`
        Ok::<_, Infallible>(service_fn(hello_world))
    });
    let server = Server::bind(&addr).serve(new_svc);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
