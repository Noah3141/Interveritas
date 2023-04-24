use{
    serde::{Deserialize},
};

#[derive(Deserialize)] // This tells serde that an endpoint can receive JSON of format '{"list":"TextInAString"}', and to translate that into THIS struct for my Rust handlers
pub struct PastedList {
    pub list: String,
}