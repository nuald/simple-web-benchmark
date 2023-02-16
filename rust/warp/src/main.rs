use getopts::Options;
use std::{env, fs, process};
use warp::hyper::StatusCode;
use warp::{self, path, reply, Filter, Rejection, Reply};

async fn handle_not_found(err: Rejection) -> Result<impl Reply, Rejection> {
    if err.is_not_found() {
        Ok(reply::with_status("NOT_FOUND", StatusCode::NOT_FOUND))
    } else {
        eprintln!("unhandled rejection: {err:?}");
        Ok(reply::with_status(
            "INTERNAL_SERVER_ERROR",
            StatusCode::INTERNAL_SERVER_ERROR,
        ))
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
    println!("Master {pid} is running on port {port}");

    let index = path::end().map(|| "Hello, World!");
    let greeting = warp::path!("greeting" / String).map(|name| format!("Hello, {name}"));

    let routes = warp::get()
        .and(index.or(greeting))
        .recover(handle_not_found);
    warp::serve(routes).run(([127, 0, 0, 1], port)).await;
}
