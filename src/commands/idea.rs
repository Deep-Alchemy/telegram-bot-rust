// Imports
use teloxide::prelude::*;

use crate::{
    utils::fetch_api,
    Cxt,
    models::IdeaApiResponse
};

pub async fn idea_command(cx: &Cxt) -> Message {
    let data = fetch_api::<IdeaApiResponse>("https://itsthisforthat.com/api.php?json").await;

    match data {
        Ok(idea) => {
            cx.answer(format!("{} for {}", idea.this, idea.that)).await.unwrap()
        },
        Err(err) => {
            let _ = err;
            cx.answer("Beep boop, No idea found!").await.unwrap()
        }
    }
}
