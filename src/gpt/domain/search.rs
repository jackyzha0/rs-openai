use serde::{Serialize, Deserialize};
use anyhow::Result;
use reqwest::Response;

#[derive(Serialize, Debug)]
pub struct Options {
    /// Up to 200 documents to search over, provided as a list of strings.
    //
    // The maximum document length (in tokens) is 2034 minus the number of tokens in the query.
    pub documents: Vec<String>,

    /// Query to search against the documents.
    pub query: String
}

/// represents a response structure for search API
#[derive(Serialize, Deserialize, Debug)]
pub struct SearchResponse {
    data: Vec<Search>
}

/// represents a single possible search
#[derive(Serialize, Deserialize, Debug)]
pub struct Search {
    document: u16,
    score: f32,
}