//! HTTP server example with hyper in compatible mode.

use getopts::Options;
use regex::Regex;
use std::net::SocketAddr;
use std::{env, fs, process, thread};

use futures::Future;
use hyper::{server::conn::Http, service::service_fn};
use hyper::{Body, Request, Response, StatusCode};
use monoio::net::TcpListener;
use monoio_compat::TcpStreamCompat;

lazy_static::lazy_static! {
    static ref GREETING_RE: Regex = Regex::new(r"^/greeting/([a-z]+)$").unwrap();
}

#[derive(Clone)]
struct HyperExecutor;

impl<F> hyper::rt::Executor<F> for HyperExecutor
where
    F: Future + 'static,
    F::Output: 'static,
{
    fn execute(&self, fut: F) {
        monoio::spawn(fut);
    }
}

async fn serve_http<S, F, R, A>(addr: A, service: S) -> std::io::Result<()>
where
    S: FnMut(Request<Body>) -> F + 'static + Copy,
    F: Future<Output = Result<Response<Body>, R>> + 'static,
    R: std::error::Error + 'static + Send + Sync,
    A: Into<SocketAddr>,
{
    let listener = TcpListener::bind(addr.into())?;
    loop {
        let (stream, _) = listener.accept().await?;
        monoio::spawn(
            Http::new()
                .with_executor(HyperExecutor)
                .serve_connection(TcpStreamCompat::new(stream), service_fn(service)),
        );
    }
}

async fn hyper_handler(req: Request<Body>) -> Result<Response<Body>, std::convert::Infallible> {
    match req.uri().path() {
        "/" => Ok(Response::new(Body::from("Hello World!"))),
        path => match GREETING_RE.captures(path) {
            Some(cap) => Ok(Response::new(Body::from(format!("Hello, {}", &cap[1])))),
            None => Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::from("404 Not Found\n"))
                .unwrap()),
        },
    }
}

fn main() {
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

    let threads_num: usize = thread::available_parallelism().unwrap().into();
    let threads: Vec<_> = (0..threads_num)
        .map(|_| {
            thread::spawn(move || {
                let mut rt = monoio::RuntimeBuilder::<monoio::FusionDriver>::new()
                    .build()
                    .unwrap();
                rt.block_on(async move { serve_http(([0, 0, 0, 0], port), hyper_handler).await })
            })
        })
        .collect();

    threads.into_iter().for_each(|t| {
        let _ = t.join().unwrap();
    });
}
