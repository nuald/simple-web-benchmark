#![feature(proc_macro_hygiene, decl_macro)]

extern crate libc;
#[macro_use] extern crate rocket;
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

    let app = rocket::custom(config);
    app
        .mount("/",
            routes![
                index,
                greeting
            ]
        )
        .launch();
}
