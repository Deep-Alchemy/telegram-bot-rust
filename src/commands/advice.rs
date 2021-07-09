// Imports
use teloxide::prelude::*;

use crate::{
    utils::fetch_api,
    Cxt,
    models::AdviceAPIResponse
};

pub async fn advice_command(cx: &Cxt) -> Message {
    let data = fetch_api::<AdviceAPIResponse>("https://api.adviceslip.com/advice").await;
    match data {
        Ok(advice) => {
            cx.answer(advice.slip.advice).await.unwrap()
        },
        Err(err) => {
            let _ = err;
            cx.answer("It looks like our server doesn't wants to give you any advice ;)").await.unwrap()
        }
    }
}
