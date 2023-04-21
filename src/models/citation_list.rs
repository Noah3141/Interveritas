/* Citation List */
use{
    serde::{Deserialize, Serialize},
};
#[derive(Deserialize, Serialize)]
pub struct CitationList {
    pub list: Vec<String>,
    pub disciplines: Vec<String>,
    pub average_year: u16,
    pub top_journals: Vec<String>, // We want this to be sorted
    pub top_authors: Vec<String>, // We want this to be sorted
    pub article_id: Option<u32>, // FOREIGN KEY of article which CONTAINS this list
    pub id: Option<u32> // PRIMARY KEY
}
