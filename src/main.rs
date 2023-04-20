#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch] // Also can use  #[rocket::main] and add {let _rocket = ... .launch().await?;    Ok(()) }
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
}
