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

/* Citation List */
#[derive(Deserialize, Serialize)]
pub struct CitationList {
    list: Vec<String>,
    disciplines: Vec<String>,
    average_year: u16,
    top_journals: Vec<String>, // We want this to be sorted
    top_authors: Vec<String>, // We want this to be sorted
    article_id: Option<u32>, // FOREIGN KEY of article which CONTAINS this list
    id: u32 // PRIMARY KEY
}

/* Author */
#[derive(Deserialize, Serialize)]
pub struct Author {
    pub name: String,
    pub abbrief: String,
    pub articles: Vec<Citation>, // Contains FOREIGN KEYS
    pub disciplines: Vec<String>, 
    pub study_dist: Vec<[f8; mem::variant_count::<Paradigm>()]>, // Ensures this is a vector of the same length as the number of variants in Paradigm
    pub id: u32,
}

/* Journal */
#[derive(Deserialize, Serialize)]
pub struct Journal {
    list: Vec<String>,
    average_year: u16, // todo: Add cooler metrics for Journals, that better detect spurts in popularity/productivity (this won't store ALL articles, just article in the database, which is a proxy for relevance)
    external_cites: u32, // Metric which measures how much, how frequently, the journal's articles cite articles in other journals
    top_authors: Vec<String>,
    disciplines: Vec<String>,
    interdiscipline: u32, // Metric of how much articles in this journal have lists of mixed disciplines
    id: u32 // PRIMARY KEY
// todo: Calculation of interdiscipline: by the article, aggregate across articles
} 


#[derive(Deserialize, Serialize)]
pub struct Topic {
    disciplines: Vec<String>,
    articles: Vec<Citation>,
    id: u32,
}



/* 
    While the other fields of the Citation struct are intended to abstract UP
    the information of the article, this field holds a struct that will analyze DOWN
    into the article, so that the API is able to scale the hierarchy to provide genuine
    organization of research. 
*/
pub struct Analysis { 
    critiques: String,
    paradigm_analysis: Paradigm_Analysis


}

pub struct Paradigm_Analysis {
    paradigm_type: String,
    paradigm_subtype: String,
    logical_thread: String,
    hypothetico_deductivity: String,
    counterfactual_verification: String,
}