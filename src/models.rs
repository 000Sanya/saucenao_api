use std::{
    collections::HashMap,
};

use serde_derive::{Serialize, Deserialize};
use serde_json::Value;

use crate::maybe::Maybe;

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    header: Header,
    results: Vec<SearchResult>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Header {
    pub user_id: Maybe<i64>,
    pub account_type: Maybe<i64>,
    pub short_limit: Maybe<i64>,
    pub short_remaining: i64,
    pub long_limit: Maybe<i64>,
    pub long_remaining: i64,
    pub status: i64,
    pub result_requested: Option<i64>,
    pub search_depth: Maybe<i64>,
    pub minimum_similarity: f64,
    pub query_image_display: String,
    pub query_image: String,
    pub results_returned: i64,
    pub index: HashMap<String, Index>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Index {
    pub status: i64,
    #[serde(default)]
    pub parend_id: Option<i64>,
    pub id: i64,
    #[serde(default)]
    pub results: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SearchResult {
    pub header: ResultHeader,
    pub data: ResultData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResultHeader {
    pub similarity: Option<Maybe<f64>>,
    pub trumbnail: Option<String>,
    pub index_id: Option<i64>,
    pub index_name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResultData {
    #[serde(default)]
    pub ext_urls: Vec<String>,
    #[serde(flatten)]
    pub _extra: HashMap<String, Value>,
}