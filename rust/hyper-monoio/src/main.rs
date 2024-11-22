//! HTTP server example with hyper in compatible mode.

use getopts::Options;
use regex::Regex;
use std::convert::Infallible;
use std::net::SocketAddr;
use std::{env, fs, process, thread};

use futures::Future;
use http_body_util::Full;
use hyper::body::Bytes;
use hyper::{server::conn::http1, service::service_fn};
use hyper::{Request, Response, StatusCode};
use monoio::{io::IntoPollIo, net::TcpListener};

lazy_static::lazy_static! {
    static ref GREETING_RE: Regex = Regex::new(r"^/greeting/([a-z]+)$").unwrap();
}

async fn serve_http<S, F, E, A>(addr: A, service: S) -> std::io::Result<()>
where
    S: Copy + Fn(Request<hyper::body::Incoming>) -> F + 'static,
    F: Future<Output = Result<Response<Full<Bytes>>, E>> + 'static,
    E: std::error::Error + 'static + Send + Sync,
    A: Into<SocketAddr>,
{
    let listener = TcpListener::bind(addr.into())?;
    loop {
        let (stream, _) = listener.accept().await?;
        let stream_poll = monoio_compat::hyper::MonoioIo::new(stream.into_poll_io()?);
        monoio::spawn(async move {
            // Handle the connection from the client using HTTP1 and pass any
            // HTTP requests received on that connection to the `hello` function
            if let Err(err) = http1::Builder::new()
                .serve_connection(stream_poll, service_fn(service))
                .await
            {
                println!("Error serving connection: {:?}", err);
            }
        });
    }
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
                rt.block_on(async move { serve_http(([0, 0, 0, 0], port), hello_world).await })
            })
        })
        .collect();

    threads.into_iter().for_each(|t| {
        let _ = t.join().unwrap();
    });
}
