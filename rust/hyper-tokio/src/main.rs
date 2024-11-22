use getopts::Options;
use http_body_util::Full;
use hyper::body::Bytes;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::StatusCode;
use hyper::{Request, Response};
use hyper_util::rt::TokioIo;
use regex::Regex;
use std::convert::Infallible;
use std::net::SocketAddr;
use std::{env, fs, process};
use tokio::net::TcpListener;

lazy_static::lazy_static! {
    static ref GREETING_RE: Regex = Regex::new(r"^/greeting/([a-z]+)$").unwrap();
}

async fn hello_world(
    req: Request<hyper::body::Incoming>,
) -> Result<Response<Full<Bytes>>, Infallible> {
    match req.uri().path() {
        "/" => Ok(Response::new(Full::new(Bytes::from("Hello World!")))),
        path => match GREETING_RE.captures(path) {
            Some(cap) => Ok(Response::new(Full::new(Bytes::from(format!(
                "Hello, {}",
                &cap[1]
            ))))),
            None => {
                let mut resp = Response::new(Full::new(Bytes::from("404 Not Found\n")));
                *resp.status_mut() = StatusCode::NOT_FOUND;
                Ok(resp)
            }
        },
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
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
    println!("Master {pid} is running on port {port}");

    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let listener = TcpListener::bind(addr).await?;

    loop {
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);
        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .serve_connection(io, service_fn(hello_world))
                .await
            {
                eprintln!("Error serving connection: {:?}", err);
            }
        });
    }
}
