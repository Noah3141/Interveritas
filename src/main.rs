mod citation;
use citation::Citation;
use citation::CitSourceType;

#[derive(serde::Deserialize)]
struct NameParams {
    pub name: String,
}


async fn handle(req: tide::Request<()>) -> tide::Result<String> {

    let mut req_url = String::from(req.url().as_str());

    let response = String::from("That's a cat.");

    Ok(format!("\nYour URL was: {} \nYour response is: {}", req_url, response))
    
}

#[async_std::main]
async fn main() -> tide::Result<()> {

    let mut app = tide::new();
    
    app.at("/").get(handle);
    app.at("/cats").get(handle);
    app.at("/dogs").get(handle);
    app.at("/cats/oscar").get(handle);
    app.at("/cat").post(handle);
    
    app.listen("0.0.0.0:8080").await?;

    Ok(())

}
