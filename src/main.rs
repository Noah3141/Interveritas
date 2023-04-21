// Bringing remote Rocket into scope
#[macro_use]
extern crate rocket;

use rocket::{get, http::Status, serde::json::Json};
// use rocket::tokio::time::{sleep, Duration}; // sleep(Duration::from_secs(delay)).await;

/* Importing local modules */ 
mod api;
mod models;
mod repository;

/* Routes */
use api::citation_api::{add_citation, get_citation_example};
use repository::mongodb_repo::MongoRepo;

#[rocket::main] // Also can use  #[rocket::main] and add {let _rocket = ... .launch().await?;    Ok(()) }
async fn main() -> Result<(), rocket::Error> {
    
    let db = MongoRepo::init().await;
    let _rocket = 
        rocket::build()
        .manage(db)
        .mount("/", routes![add_citation, get_citation_example]) // Route handles are best named "Why" the person it requesting the URL
        .launch()
        .await?;
    Ok(())
}