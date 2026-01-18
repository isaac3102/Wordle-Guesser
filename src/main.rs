use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use memory_stats::memory_stats;
use wordle_guesser::analyze_contents;
use std::thread;
use std::sync::Arc;
use wordle_guesser::{Character, main_selector, read_file};

mod test;

// Main function to run the HTTP server
/* #[actix_web::main]
async fn main() -> std::io::Result<()> {
    let content = lib::read_file().unwrap();
    analyze_contents(content);
    HttpServer::new(|| {
        App::new().route("/", web::get().to(greet)) // Route "/" to the greet function
    })
    .bind("127.0.0.1:8080")? // Bind the server to localhost:8080
    .run()
    .await
} */

fn main() {
    //let content = read_file().unwrap();
    //main_selector(content, "tribe".to_string());

    test::test_words();

    
}