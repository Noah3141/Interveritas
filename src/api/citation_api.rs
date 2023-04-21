use crate::{models::citation::Citation, repository::mongodb_repo::MongoRepo};
use mongodb::results::InsertOneResult;
use rocket::{http::Status, serde::json::Json, State};
use crate::models::citation::Identifier;
use crate::models::citation::Paradigm;

#[post("/citation", format = "application/json", data = "<new_citation>")]
pub async fn add_citation(db: &State<MongoRepo>, new_citation: Json<Citation>) -> Result<Json<InsertOneResult>, Status> {

    let data:Citation = Citation{
        ..new_citation.into_inner()
    };
    let citation_detail = db.create_citation(data).await;
    match citation_detail {
        Ok(citation) => Ok(Json(citation)), // Wrap the InsertOne result in JSON and return it to the post requester as a token of success
        Err(_) => Err(Status::InternalServerError), // If you got an error from Mongo trying to insert, throw away Error inside Err, and return a 500 status code
    }
}



#[get("/example")]
pub async fn get_citation_example() -> Result<Json<Citation>, Status> {
    let mut data = Citation::new_article(
        "DiNicolantonio, J. J., Lucan, S. C., & O’Keefe, J. H.".to_string(), 
        "2016".to_string(), 
        "The evidence for saturated fat and for sugar related to coronary heart disease".to_string(), 
        "Progress in cardiovascular diseases".to_string(), 
        "58".to_string(), 
        "5".to_string(), 
        "464-472".to_string(), 
        Identifier::Doi("https://doi.org/10.1016/j.pcad.2015.11.006".to_string()));

    data.keywords = Some(vec![String::from("cardiovascular disease"), String::from("Saturated fat"), String::from("Sugar"), String::from("Sucrose"), String::from("Fructose"), String::from("Coronary heart disease"), String::from("Fatty acids"), String::from("Lipids"), String::from("Cholesterol") ]); 
    data.disciplines = Some(vec![String::from("traditional medicine"), String::from("epidemiology"), String::from("nutrition science") ]);
    data.article_cited = Some(vec![32,76,1880,12,13]);
    data.article_cites = Some(vec![1666,1663,1664,1665,1666]);
    data.conclusions = Some(String::from(" This paper reviews the evidence linking saturated fats and sugars to CHD, and concludes that the latter is more of a problem than the former. Dietary guidelines should shift focus away from reducing saturated fat, and from replacing saturated fat with carbohydrates, specifically when these carbohydrates are refined. To reduce the burden of CHD, guidelines should focus particularly on reducing intake of concentrated sugars, specifically the fructose-containing sugars like sucrose and high-fructose corn syrup in the form of ultra-processed foods and beverages."));
    data.limitations = Some(String::from("None listed."));
    data.interest_disclosure = Some(String::from("None of the authors have any conflicts of interests with regard to this publication. Research by Dr. Lucan reported in this publication was supported by the Eunice Kennedy Shriver National Institute Of Child Health & Human Development of the National Institutes of Health under Award Number K23HD079606."));
    data.methods = Some(String::from("None listed."));
    data.paradigm = Some(Paradigm::Review(String::from("Authors collected research to make a conclusion about a topic.")));
    data.hypotheses = Some(String::from("None listed."));
    data.summary = Some(String::from(""));

    Ok(Json(data))

}
