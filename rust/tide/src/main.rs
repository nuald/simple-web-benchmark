use async_std::task;
use getopts::Options;
use std::{env, fs, process};
use tide::utils::After;
use tide::{self, Request, Response, StatusCode};

fn main() -> Result<(), std::io::Error> {
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
    println!("Master {} is running on port {}", pid, port);

    task::block_on(async {
        let mut app = tide::new();

        app.with(After(|response: Response| async move {
            let response = match response.status() {
                StatusCode::NotFound => Response::builder(404).body("404 Not Found").build(),

                StatusCode::InternalServerError => Response::builder(500)
                    .body("500 Internal Server Error")
                    .build(),

                _ => response,
            };

            Ok(response)
        }));

        app.at("/").get(|_| async { Ok("Hello, world!") });
        app.at("/greeting/:name")
            .get(|req: Request<()>| async move {
                Ok(format!("Hello, {}", req.param("name").unwrap()))
            });
        app.listen(format!("127.0.0.1:{}", port)).await?;
        Ok(())
    })
}
