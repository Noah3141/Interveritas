/* Citation List */
use{
    serde::{Deserialize, Serialize},
};
use crate::models::citation::Citation;

#[derive(Deserialize, Serialize)]
pub struct CitationList {
    pub list: Vec<Citation>,
    pub disciplines: Option<Vec<String>>,
    pub average_year: Option<u16>,
    pub top_journals: Option<Vec<String>>, // We want this to be sorted
    pub top_authors: Option<Vec<String>>, // We want this to be sorted
    pub article_id: Option<u32>, // FOREIGN KEY of article which CONTAINS this list
    pub id: Option<u32> // PRIMARY KEY
}

impl CitationList {
    pub fn new_blank() -> Self {

        CitationList { 
            list: vec![ Citation::new_blank(),], 
            disciplines: None, 
            average_year: None, 
            top_journals: None, 
            top_authors: None, 
            article_id: None, 
            id: None,
        }

    }
}
