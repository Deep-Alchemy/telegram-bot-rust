// Imports
use serde::{Deserialize, Serialize};

use serde;

#[derive(Deserialize, Debug)]
pub struct ImageAPiResponse {
    pub url: String,
}

#[derive(Deserialize, Debug)]
pub struct QuoteApiResponse {
    pub anime: String,
    pub character: String,
    pub quote: String,
}


// Advice Response
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdviceAPIResponse {
    pub slip: Slip,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Slip {
    pub id: i64,
    pub advice: String,
}
