/* Topic */
use{
    serde::{Deserialize, Serialize},
};

use crate::models::citation::Citation;

#[derive(Deserialize, Serialize)]
pub struct Topic {
    pub disciplines: Vec<String>,
    pub articles: Vec<Citation>,
    pub id: u32,
}