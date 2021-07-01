// Imports
use rand::seq::SliceRandom;

use crate::models::APiResponse;

pub async fn get_image(category: &str) -> Result<APiResponse, reqwest::Error> {
    let mut mutable_category = category;
    if mutable_category.is_empty() {
        mutable_category = "waifu"
    };

    let endpoint = format!("https://api.waifu.pics/sfw/{}", mutable_category);
    let res = reqwest::get(endpoint)
        .await
        .unwrap()
        .json::<APiResponse>()
        .await;

    res
}

pub async fn get_random_image() -> APiResponse {
    let categories = vec!["waifu", "shinobu", "neko"];
    let random_category = categories.choose(&mut rand::thread_rng()).unwrap();

    get_image(random_category).await.unwrap()
}
