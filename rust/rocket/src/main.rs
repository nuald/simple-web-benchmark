#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate libc;
extern crate rocket;

use rocket::config::{Config, Environment};
use rocket::http::RawStr;

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
    let config = Config::build(Environment::Staging)
        .address("127.0.0.1")
        .port(3000)
        .workers(256)
        .unwrap();

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
