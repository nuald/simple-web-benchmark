use getopts::Options;
use rocket::config::Config;
use std::net::Ipv4Addr;
use std::{env, fs, process};

#[rocket::get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[rocket::get("/greeting/<name>")]
fn greeting(name: &str) -> String {
    format!("Hello, {name}")
}

#[rocket::launch]
fn rocket() -> _ {
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

    let config = Config {
        port,
        address: Ipv4Addr::new(127, 0, 0, 1).into(),
        ..Config::release_default()
    };

    let app = rocket::custom(config);
    app.mount("/", rocket::routes![index, greeting])
}
