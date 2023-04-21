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
    pub identifier: Identifer,
}
#[derive(Deserialize, Serialize)]
pub enum Identifer {
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
#[derive(Deserialize, Serialize)]
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
    pub sourceType: SourceType, // enum of structs of all (strings or no strings)
    pub summary: String,
    pub conclusions: String,
    pub limitations: String,
    pub paradigm: Paradigm, // enum with variants holding Strings
    pub methods: Option<String>,
    pub hypotheses: Option<String>,
    pub funding: u64,
    pub interestDisclosure: Option<String>,
    pub keywords: Vec<String>,
    pub disciplines: Vec<String>,
    pub article_cites: Vec<u32>, // Vector of primary keys of articles that this article cites
    pub article_cited: Vec<u32>, // Vector of primary keys of artciles that cite this article
    pub journal_id: Option<u32>, // FOREIGN KEY
    pub id: u32, // PRIMARY KEY
    pub article_analysis: Option<Analysis>,
}

/* 
    While the other fields of the Citation struct are intended to abstract UP
    the information of the article, this field holds a struct that will analyze DOWN
    into the article, so that the API is able to scale the hierarchy to provide genuine
    organization of research. 
*/
pub struct Analysis { 
    pub critiques: String,
    pub paradigm_analysis: Paradigm_Analysis
}

pub struct Paradigm_Analysis {
    pub paradigm_type: String,
    pub paradigm_subtype: String,
    pub logical_thread: String,
    pub hypothetico_deductivity: String,
    pub counterfactual_verification: String,
}