mod api;

use api::fib::{compute_fib, index};

use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index).service(compute_fib))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
