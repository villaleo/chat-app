use actix_web::*;
use actix_cors::Cors;
use http::header;

mod db;
mod models;
mod routes;
mod schema;
mod server;
mod session;

const SERVER_ADDRESS: &str = "127.0.0.1";
const SERVER_PORT: u16 = 8080;
const AMOUNT_WORKERS: usize = 2;
const FRONTEND_ORIGIN: &str = "http://localhost:3000";
const BACKEND_ORIGIN: &str = "http://localhost:8080";
const CORS_MAX_REQUEST_CACHE_SEC: usize = 3600;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    todo!("Initialize server, manager, and connection pool");
    let server;
    let conn_spec = "chat.db";
    let manager;
    let pool;

    let app = HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin(FRONTEND_ORIGIN)
            .allowed_origin(BACKEND_ORIGIN)
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
            .allowed_header(header::CONTENT_TYPE)
            .max_age(CORS_MAX_REQUEST_CACHE_SEC);
        todo!("Setup the app routes and data");
        App::new()
    })
    .workers(AMOUNT_WORKERS)
    .bind((SERVER_ADDRESS, SERVER_PORT)) ?
    .run();
    println!("Running at http://{SERVER_ADDRESS}:{SERVER_PORT}");
    app.await
}
