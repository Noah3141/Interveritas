// Bringing remote Rocket into scope
#[macro_use]
extern crate rocket;

use api::citation_list_api::google_me_list;
use rocket::{get, http::Status, serde::json::Json};
// use rocket::tokio::time::{sleep, Duration}; // sleep(Duration::from_secs(delay)).await;

mod api;
mod models;
mod repository;
mod utils;

use api::citation_api::*;
use api::citation_list_api::*;
use repository::mongodb_repo::MongoRepo;



#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    
    let db = MongoRepo::init().await;
    let _rocket = 
        rocket::build()
        .manage(db)
        .mount("/", routes![
            add_citation,
            get_citation_example, 
            google_me_list, 
            parse_me_list
            ]) // Route handles are best named "Why" the person it requesting the URL
        .launch()
        .await?;
    Ok(())
}