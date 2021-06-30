// Imports
use teloxide::prelude::*;

use crate::utils::get_random_image;

pub async fn random_command(cx: UpdateWithCx<AutoSend<Bot>, Message>) -> Message {
    let random_image = get_random_image().await;

    cx.answer_photo(teloxide::types::InputFile::Url(random_image.url))
        .await.unwrap()
}
