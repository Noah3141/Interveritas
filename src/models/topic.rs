/* Topic */
#[derive(Deserialize, Serialize)]
pub struct Topic {
    pub disciplines: Vec<String>,
    pub articles: Vec<Citation>,
    pub id: u32,
}