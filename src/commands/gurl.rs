// Imports
use teloxide::prelude::*;

use crate::utils::get_image;

pub async fn gurl_command(cx: UpdateWithCx<AutoSend<Bot>, Message>, category: String) -> Message {
    let image = get_image(&category).await;

    cx.answer_photo(teloxide::types::InputFile::Url(image.url))
        .await.unwrap()
}
