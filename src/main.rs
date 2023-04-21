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
#[get("/", rank = 2)]
async fn greet_me() -> Result<Json<String>, Status> {
    
    Ok(Json(String::from("Hello, mongoDB!")))

}


/* App */
#[launch] // Also can use  #[rocket::main] and add {let _rocket = ... .launch().await?;    Ok(()) }
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![greet_me]) // Route handles are best named "Why" the person it requesting the URL
}
