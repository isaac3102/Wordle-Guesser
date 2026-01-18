use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use memory_stats::memory_stats;
use wordle_guesser::analyze_contents;
use std::thread;
use std::sync::Arc;
use wordle_guesser::{Character, main_selector, read_file};

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
    let content = Arc::new(read_file().unwrap());
    main_selector(content, "after".to_string());


    /* let handle = thread::spawn(move || {

        println!("Memory Usage: {:?}", memory_stats::memory_stats().unwrap());

        for i in 0..26{
    
            let content = Arc::clone(&content);
            let ranking= Arc::clone(&ranking);

            //println!("analyzing one level {:?}", i);
            //println!("{:?}", entry_pattern(content, ranking, i, 26));
            main_selector(content, ranking, "apple".to_string());

        } // thread (.join() inherent function is used to ensure that program waits for thread to finish) otherwise can use a handle variable to keep track of all threads?

    });

    handle.join().expect("worker thread panicked"); */
    
}