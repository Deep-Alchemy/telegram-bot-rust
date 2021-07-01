// Imports
use teloxide::prelude::*;

use crate::{
    utils::get_image,
    Cxt
};

pub async fn gurl_command(cx: &Cxt, category: String) -> Message {
    let image = get_image(&category).await;

    cx.answer_photo(teloxide::types::InputFile::Url(image.url))
        .await
        .unwrap()
}
