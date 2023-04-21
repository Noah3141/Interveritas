/* Journal */
use{
    serde::{Deserialize, Serialize},
};
#[derive(Deserialize, Serialize)]
pub struct Journal {
    pub list: Vec<String>,
    pub average_year: u16, // todo: Add cooler metrics for Journals, that better detect spurts in popularity/productivity (this won't store ALL articles, just article in the database, which is a proxy for relevance)
    pub external_cites: u32, // Metric which measures how much, how frequently, the journal's articles cite articles in other journals
    pub top_authors: Vec<String>,
    pub disciplines: Vec<String>,
    pub interdiscipline: u32, // Metric of how much articles in this journal have lists of mixed disciplines
    pub id: Option<u32> // PRIMARY KEY
// todo: Calculation of interdiscipline: by the article, aggregate across articles
} 