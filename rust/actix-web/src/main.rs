use actix_web::web::{self, Path};
use actix_web::{App, HttpResponse, HttpServer};
use getopts::Options;
use std::io::Result;
use std::{env, fs, process};

#[actix_web::main]
async fn main() -> Result<()> {
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

    HttpServer::new(|| {
        App::new()
            .service(web::resource("/").to(|| async {
                HttpResponse::Ok()
                    .content_type("text/plain")
                    .body("Hello world!")
            }))
            .service(
                web::resource("/greeting/{name}").to(|path: Path<String>| async move {
                    HttpResponse::Ok()
                        .content_type("text/plain")
                        .body(format!("Hello {path}!"))
                }),
            )
            .default_service(web::to(|| async {
                HttpResponse::Ok()
                    .content_type("text/plain")
                    .body("404 Not Found")
            }))
    })
    .bind(format!("0.0.0.0:{port}"))?
    .run()
    .await
}
