mod api;

use api::fib::{compute_fib, index};

use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = std::env::var("SERVER_PORT").expect("SERVER_PORT must be set");
    let server = HttpServer::new(|| App::new().service(index).service(compute_fib))
        .bind(("127.0.0.1", port.parse::<u16>().unwrap()))?;

    println!(
        "Starting `Simple Rust Api` server, listening on port {}",
        port
    );

    server.run().await
}
