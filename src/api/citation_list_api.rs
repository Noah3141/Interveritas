use crate::{models::citation::Citation, repository::mongodb_repo::MongoRepo};
use mongodb::results::InsertOneResult;
use rocket::{http::Status, serde::json::Json, State};
use crate::models::citation::Identifier;
use crate::models::citation::Paradigm;
use crate::models::citation_list::CitationList;
use crate::utils::google_scholar_list::google_scholar_list;
use crate::utils::parse_list::parse_list;


#[post("/google-scholar-list", format = "application/json", data = "<prompt>")]
pub async fn google_scholar_list(db: &State<MongoRepo>, prompt: Json<String>,) -> Result<Json<CitationList>, Status> {

    //   Google generate list
    prompt = prompt.into_inner();
    let mut citation_list: CitationList = google_scholar_list(prompt); 
    

    // Save it to my database


    // Give the person their list on front end
    Ok(Json(<CitationList>))
}

#[post("/parse-list", format = "application/json", data = "<pasted_list>")]
pub async fn add_citation(db: &State<MongoRepo>, pasted_list: Json<String>) -> Result<Json<CitationList>, Status> {
    
    let pasted_list: String = pasted_list.into_inner(); // De-JSON the list into a String

    let mut citation_list = parse_list(pasted_list); // todo Does this need to return an Option? Something to pass the signature's "Status", if err?

    Ok(Json(<CitationList>))
}
