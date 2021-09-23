#![feature(decl_macro)]

use getopts::Options;
use rocket::config::{Config, Environment};
use rocket::http::RawStr;
use std::{env, fs, process};

#[rocket::get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[rocket::get("/greeting/<name>")]
fn greeting(name: &RawStr) -> String {
    format!("Hello, {}", name)
}

fn main() {
    let pid = process::id().to_string();
    fs::write(".pid", &pid).expect("Unable to write file");

    let args = env::args().collect::<Vec<String>>();
    let mut opts = Options::new();
    opts.optflag("p", "prod", "use the vanilla production config");
    opts.optopt("", "port", "server port", "PORT");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!("{}", f),
    };

    let port = matches.opt_get::<u16>("port").unwrap().unwrap_or(3000);

    println!("Master {} is running on port {}", pid, port);

    let mut config_builder = Config::build(Environment::Production)
        .address("127.0.0.1")
        .port(port);

    if !matches.opt_present("p") {
        config_builder = config_builder.workers(256);
    }

    let config = config_builder.unwrap();

    let app = rocket::custom(config);
    app.mount("/", rocket::routes![index, greeting]).launch();
}
