// Imports
use teloxide::prelude::*;

use crate::{
    utils::use_api,
    Cxt,
    models::IdeaApiResponse
};

pub async fn idea_command(cx: &Cxt) -> Message {
    let data = use_api::<IdeaApiResponse>("https://itsthisforthat.com/api.php?json").await;

    match data {
        Ok(idea) => {
            cx.answer(format!("{} for {}", idea.this, idea.that)).await.unwrap()
        },
        Err(err)=> {
            let _ = err;
            cx.answer("It looks like our server doesn't wants to give you any advice ;)").await.unwrap()
        }
    }
}
