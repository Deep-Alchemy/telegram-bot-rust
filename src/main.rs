use reqwest;
use teloxide::{payloads::SendPhotoSetters, prelude::*};
use serde::Deserialize;

#[tokio::main]
async fn main() {
    run().await
}

#[derive(Deserialize, Debug)]
struct APiResponse {
    url: String,
}

async fn run() {
    teloxide::enable_logging!();
    log::info!("Starting BOT!!!");

    let bot = Bot::new("Your TOKEN").auto_send();

    teloxide::repl(bot, |message| async move {
        let res = reqwest::get("https://api.waifu.pics/sfw/shinobu")
            .await
            .unwrap()
            .json::<APiResponse>()
            .await
            .unwrap();

        message
            .answer_photo(teloxide::types::InputFile::Url(res.url))
            .caption("Hello Shinobu")
            .await?;
        respond(())
    })
    .await;
}
