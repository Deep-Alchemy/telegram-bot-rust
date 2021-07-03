// Imports
use rand::seq::SliceRandom;

use crate::models::ImageAPiResponse;

pub async fn get_image(category: &str) -> Result<ImageAPiResponse, reqwest::Error> {
    let mut mutable_category = category;
    if mutable_category.is_empty() {
        mutable_category = "waifu"
    };

    let endpoint = format!("https://api.waifu.pics/sfw/{}", mutable_category);
    let res = reqwest::get(endpoint)
        .await
        .unwrap()
        .json::<ImageAPiResponse>()
        .await;

    res
}

pub async fn get_random_image() -> ImageAPiResponse {
    let categories = vec!["waifu", "shinobu", "neko"];
    let random_category = categories.choose(&mut rand::thread_rng()).unwrap();

    get_image(random_category).await.unwrap()
}

// Use this instead of writing reqwest everytime
pub async fn use_api<T : serde::de::DeserializeOwned>(endpoint: &str) -> Result<T, reqwest::Error> {
    let res = reqwest::get(endpoint).await.unwrap().json::<T>().await;
    res
}
