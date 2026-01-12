use actix_web::{web, App, HttpResponse, HttpServer, Responder};
mod lib;
use memory_stats::memory_stats;
use wordle_guesser::analyze_contents;
use std::mem::drop;

// Define the greet function
async fn greet() -> impl Responder {
    HttpResponse::Ok().body("Hello from Rust!")
}

// Main function to run the HTTP server
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let content = lib::read_file().unwrap();
    analyze_contents(content);
    HttpServer::new(|| {
        App::new().route("/", web::get().to(greet)) // Route "/" to the greet function
    })
    .bind("127.0.0.1:8080")? // Bind the server to localhost:8080
    .run()
    .await
}