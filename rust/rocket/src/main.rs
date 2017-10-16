#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate libc;
extern crate rocket;
extern crate getopts;

use getopts::Options;
use rocket::config::{Config, Environment};
use rocket::http::RawStr;
use std::env;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/greeting/<name>")]
fn greeting(name: &RawStr) -> String {
    format!("Hello, {}", name)
}

fn main() {
    println!("Master {} is running", unsafe { libc::getpid() });

    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.optflag("p", "prod", "use the vanilla production config");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    let mut config_builder = Config::build(Environment::Production)
        .address("127.0.0.1")
        .port(3000);

    if !matches.opt_present("p") {
        config_builder = config_builder.workers(256);
    }

    let config = config_builder.unwrap();

    let app = rocket::custom(config, false);
    app
        .mount("/",
            routes![
                index,
                greeting
            ]
        )
        .launch();
}
