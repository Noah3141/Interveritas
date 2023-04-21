/* Author */
use{
    serde::{Deserialize, Serialize},
};

use crate::models::citation::Citation;
use crate::models::citation::Paradigm;

#[derive(Deserialize, Serialize)]
pub struct Author {
    pub name: String,
    pub abbrief: String,
    pub articles: Vec<Citation>, // Contains FOREIGN KEYS
    pub disciplines: Vec<String>, 
    pub study_dist: Vec<[f32; 6]>, // Ensures this is a vector of the same length as the number of variants in Paradigm
    pub id: u32,
}