use rand::seq::SliceRandom;
use reqwest;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct APiResponse {
    pub url: String,
}

pub async fn get_image(category: &str) -> APiResponse {
    let mut mutable_category = category;
    if mutable_category.is_empty() {
        mutable_category = "waifu"
    };

    let endpoint = format!("https://api.waifu.pics/sfw/{}", mutable_category);
    let res = reqwest::get(endpoint)
        .await
        .unwrap()
        .json::<APiResponse>()
        .await
        .unwrap();

    res
}

pub async fn get_random_image() -> APiResponse {
    let categories = vec!["waifu", "shinobu", "neko"];
    let random_category = categories.choose(&mut rand::thread_rng()).unwrap();

    get_image(random_category).await
}
