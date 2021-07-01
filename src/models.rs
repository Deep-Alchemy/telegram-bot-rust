// Imports
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct APiResponse {
    pub url: String,
}

#[derive(Deserialize, Debug)]
pub struct QuoteApiResponse {
    pub anime: String,
    pub character: String,
    pub quote: String,
}
