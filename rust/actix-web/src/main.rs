use actix_web::{web, App, HttpResponse, HttpServer};

fn main() -> std::io::Result<()> {
    let pid = unsafe { libc::getpid() }.to_string();
    std::fs::write(".pid", &pid).expect("Unable to write file");
    println!("Master {} is running", pid);

    HttpServer::new(|| {
        App::new()
            .service(web::resource("/").to(|| {
                HttpResponse::Ok()
                    .content_type("text/plain")
                    .body("Hello world!")
            }))
            .service(web::resource("/greeting/{name}").to(|path: web::Path<(String,)>| {
                HttpResponse::Ok()
                    .content_type("text/plain")
                    .body(format!("Hello {}!", path.0))
            }))
    })
    .bind("127.0.0.1:3000")?
    .run()
}
