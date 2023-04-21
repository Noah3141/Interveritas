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