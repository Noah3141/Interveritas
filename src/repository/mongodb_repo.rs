use std::{env, sync::Arc};
extern crate dotenv;
use dotenv::dotenv;


use mongodb::{
    bson::{extjson::de::Error, doc, Document},
    results::{ InsertOneResult},
    Client,
    Collection,
    options::{ClientOptions, FindOptions,}
};

use crate::models::citation::*;

use crate::models::*;

pub struct MongoRepo {
    col: Collection<Citation>
}

impl MongoRepo {
    pub async fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("MONGOURI") {
            Ok(v) => v.to_string(),
            Err(e) => format!("Couldn't resolve enviroment variables: {e}")
        };
        let client = Client::with_uri_str(uri).await.expect("Couldn't resolve client!");   
        let db = client.database("interveritas");
        let col: Collection<Citation> = db.collection("Citation");
        MongoRepo { col }
    }


    pub async fn create_citation(&self, new_citation: Citation) -> Result<InsertOneResult, Error> {
        let new_doc = Citation {
            source_type: new_citation.source_type,
            summary: new_citation.summary, 
            conclusions: new_citation.conclusions,
            limitations: new_citation.limitations, 
            paradigm: new_citation.paradigm,  
            methods: new_citation.methods, 
            hypotheses: new_citation.hypotheses,
            funding: new_citation.funding,
            interest_disclosure: new_citation.interest_disclosure,
            keywords: new_citation.keywords,
            disciplines: new_citation.disciplines,
            article_cites: new_citation.article_cites, 
            article_cited: new_citation.article_cited,  
            journal_id: new_citation.journal_id,
            article_analysis: new_citation.article_analysis,
            id: None,
            };

            let citation = self
                .col
                .insert_one(new_doc, None)
                .await
                .ok()
                .expect("Error creating citation!");
            Ok(citation)
            
        }
}