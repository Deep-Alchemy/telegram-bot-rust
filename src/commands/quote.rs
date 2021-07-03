// Imports
use rand::seq::SliceRandom;
use teloxide::{
    payloads::SendMessageSetters,
    prelude::*,
    types::ParseMode::Html
};

use crate::{
    models::QuoteApiResponse, Cxt
};

// Util functions
async fn get_random_quote() -> Result<QuoteApiResponse, reqwest::Error> {
    let res = reqwest::get("https://animechan.vercel.app/api/random")
        .await
        .unwrap()
        .json::<QuoteApiResponse>()
        .await;
    res
}

async fn get_quote_by_char(character: &str) -> Result<Vec<QuoteApiResponse>, reqwest::Error> {
    let endpoint = format!(
        "https://animechan.vercel.app/api/quotes/character?name={}",
        character
    );

    let res = reqwest::get(endpoint)
        .await
        .unwrap()
        .json::<Vec<QuoteApiResponse>>()
        .await;
    res
}

async fn send_quote(cx: &Cxt, quote: &QuoteApiResponse) -> Message {
    let text = format!("<i>{}</i> \n - <b>{}</b>", quote.quote, quote.character);
    cx.answer(text).parse_mode(Html).await.unwrap()
}

pub async fn quote_command(cx: &Cxt, character: String) -> Message {
    // If no character name is provided, send a random quote.
    if character.is_empty() {
        let res = get_random_quote().await;
        match res {
            Ok(data) => send_quote(cx, &data).await,
            Err(err) => {
                let _ = err;
                cx.answer("Some Error Over here").await.unwrap()
            }
        }
    } else {
        // Else find a quote based on the character.
        let res = get_quote_by_char(character.as_str()).await;
        match res {
            Ok(data) => {
                let single_quote = data.choose(&mut rand::thread_rng()).unwrap();
                send_quote(cx, single_quote).await
            }
            Err(err) => {
                let _ = err;
                // If not found, Send an error message.
                cx.answer("Looks like the character doesn't exists in our Database.")
                    .await
                    .unwrap()
            }
        }
    }
}
