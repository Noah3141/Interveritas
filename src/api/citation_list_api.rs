use crate::{models::citation::Citation, repository::mongodb_repo::MongoRepo};
use mongodb::results::InsertOneResult;
use rocket::{http::Status, serde::json::Json, State};
use crate::models::citation::Identifier;
use crate::models::citation::Paradigm;
use crate::models::pasted_list::PastedList;
use crate::models::citation_list::CitationList;
use crate::utils::google_scholar_list::google_scholar_list;
use crate::utils::parse_list::parse_list;



#[post("/google-scholar-list", format = "application/json", data = "<prompt>")]
pub async fn google_me_list(db: &State<MongoRepo>, prompt: Json<String>) -> Result<Json<CitationList>, Status> {

    //   Google generate list
    let prompt: String = prompt.into_inner().to_string();
    let mut citation_list: CitationList = google_scholar_list(prompt); 
    

    // Save it to my database


    // Give the person their list on front end
    Ok(Json(citation_list))
}

#[post("/parse-list", format = "application/json", data = "<pasted_list>")] // todo: Front end needs to parse inputted list carefully, so that it fits into a proper JSON request
pub async fn parse_me_list(db: &State<MongoRepo>, pasted_list: Json<PastedList>) -> Result<Json<CitationList>, Status> {
    
    let pasted_list: String = pasted_list
                                        .into_inner() // PastedList struct
                                        .list; // String
                                        // De-JSON the body using PastedList as a manual, give me the struct with into_inner, then give me the struct's field

    let mut citation_list = parse_list(pasted_list); // todo Does this need to return an Option? Something to pass the signature's "Status", if err?

    Ok(Json(citation_list))
}
