use std::env;
extern crate dotenv;
use dotenv::dotenv;

use mongodb::{
    bson::{extjson::de::Error, doc, Document},
    results::{ InsertOneResult},
    Client,
    options::{ClientOptions, FindOptions}
};

use crate::models::*;

pub struct MongoRepo {
    col: Collection<Citation>
}