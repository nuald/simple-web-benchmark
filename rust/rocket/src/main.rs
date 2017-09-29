#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

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
    rocket::ignite()
        .mount("/",
            routes![
                index,
                greeting
            ]
        )
        .launch();
}
