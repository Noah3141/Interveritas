use{
    serde::{Deserialize, Serialize},
};

/* Article */
#[derive(Deserialize, Serialize)]
pub struct Article {
    pub authors: String,
    pub year: String,
    pub title: String,
    pub journal: String,
    pub volume: String,
    pub issue: String,
    pub pages: String,
    pub identifier: Identifier,
}

#[derive(Deserialize, Serialize)]
pub enum Identifier {
    Doi(String),
    Other(String),
    None,
}

/* Book */
#[derive(Deserialize, Serialize)]
pub struct Book {
    pub authors: String,
    pub year: String,
    pub title: String,
    pub publisher: String,
    pub chapter: Option<String>,
    pub pages: Option<String>,
    pub doi: Option<String>,
}

/* Video */
#[derive(Deserialize, Serialize)]
pub struct Video {
    pub authors: String,
    pub username: String,
    pub year: String,
    pub month: String,
    pub day: String,
    pub title: String,
    pub website: String,
    pub url: Option<String>,
}

/* Webpage */
#[derive(Deserialize, Serialize)]
pub struct Webpage {
    pub authors: String,
    pub year: String,
    pub title: String,
    pub website: String,
    pub url: Option<String>,
}

/* Source Type */
#[derive(Deserialize, Serialize)]
pub enum SourceType { // Build purely containing Strings, since this is what builds reference Strings
        Article(Article),
        Book(Book),
        Video(Video),
        Webpage(Webpage),
}

/* Paradigm */
#[derive(Deserialize, Serialize, Clone)]
pub enum Paradigm {
    Experiment(String),
    Observational(String),
    Review(String),
    MetaReview(String),
    CaseStudy(String),
    Editorial(String),
    Other(String)
}

/* 
Putting it all together: CITATION 
*/
#[derive(Deserialize, Serialize)]
pub struct Citation {
    pub source_type: Option<SourceType>, // enum of structs of all (strings or no strings)
    pub summary: Option<String>,
    pub conclusions: Option<String>,
    pub limitations: Option<String>,
    pub paradigm: Option<Paradigm>, // enum with variants holding Strings
    pub methods: Option<String>,
    pub hypotheses: Option<String>,
    pub funding: Option<u64>,
    pub interest_disclosure: Option<String>,
    pub keywords: Option<Vec<String>>,
    pub disciplines: Option<Vec<String>>,
    pub article_cites: Option<Vec<u32>>, // Vector of primary keys of articles that this article cites
    pub article_cited: Option<Vec<u32>>, // Vector of primary keys of artciles that cite this article
    pub journal_id: Option<u32>, // FOREIGN KEY
    pub id: Option<u32>, // PRIMARY KEY
    pub article_analysis: Option<Analysis>,
}

/* 
    While the other fields of the Citation struct are intended to abstract UP
    the information of the article, this field holds a struct that will analyze DOWN
    into the article, so that the API is able to scale the hierarchy to provide genuine
    organization of research. 
*/
#[derive(Deserialize, Serialize, Clone)]
pub struct Analysis { 
    pub critiques: String,
    pub paradigm_analysis: ParadigmAnalysis
}
#[derive(Deserialize, Serialize, Clone)]
pub struct ParadigmAnalysis {
    pub paradigm_type: String,
    pub paradigm_subtype: String,
    pub logical_thread: String,
    pub hypothetico_deductivity: String,
    pub counterfactual_verification: String,
}

/* IMPLEMENTATIONS */

impl Citation {
    pub fn new_blank() -> Self {
        Citation{

            source_type: None, 
            summary: None,
            conclusions: None,
            limitations: None,
            paradigm: None, 
            methods: None,
            hypotheses: None,
            funding: None,
            interest_disclosure: None,
            keywords: None,
            disciplines: None,
            article_cites: None, 
            article_cited: None,
            journal_id: None,
            id: None,
            article_analysis: None,

        }  
    }
    pub fn new_article(authors:String, year:String, title:String, journal:String, volume: String, issue: String, pages: String, identifier: Identifier) -> Self {
        Citation {
            source_type: Some(SourceType::Article( Article {
                authors,
                year,
                title,
                journal,
                volume,
                issue,
                pages,
                identifier,
            })),
            summary: None, // Here, none will represent no data filled in, whereas a valid String containing "None listed." will signify the data has been collected as empty.
            conclusions: None,
            limitations: None,
            paradigm: None,
            methods: None,
            hypotheses: None,
            funding: None,
            interest_disclosure: None,
            keywords: None,
            disciplines: None,
            article_cites: None,
            article_cited: None,
            journal_id: None,
            id: None,
            article_analysis: None,

        }
    }

}

/* Article {
    pub authors: String,
    pub year: String,
    pub title: String,
    pub journal: String,
    pub volume: String,
    pub issue: String,
    pub pages: String,
    pub identifier: Identifer, */