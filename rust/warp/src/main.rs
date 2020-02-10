use warp::Filter;

#[tokio::main]
async fn main() {
    let pid = std::process::id().to_string();
    std::fs::write(".pid", &pid).expect("Unable to write file");
    println!("Master {} is running", pid);

    let index = warp::path::end().map(|| "Hello, World!");
    let greeting = warp::path!("greeting" / String).map(|name| format!("Hello, {}", name));

    let routes = warp::get().and(index.or(greeting));
    warp::serve(routes).run(([127, 0, 0, 1], 3000)).await;
}
