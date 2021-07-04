// Imports
use teloxide::prelude::*;

use crate::{Cxt, models::ImageAPiResponse, utils::{get_image, is_gif}};

// Util function to check if its a gif and send image too
pub async fn send_image(cx: &Cxt, image: ImageAPiResponse ) -> Message {
    if is_gif(image.url.as_str()) {
        cx.answer_animation(teloxide::types::InputFile::Url(image.url)).await.unwrap()
    } else {
        cx.answer_photo(teloxide::types::InputFile::Url(image.url))
            .await
            .unwrap()
    }
}

pub async fn gurl_command(cx: &Cxt, category: String) -> Message {
    let data = get_image(&category).await;

    match data {
        Ok(image) => {
            send_image(cx, image).await
        }
        Err(err) => {
            let _ = err;
            cx.answer(
                r#"
Looks like you have entered a wrong category.
Consider using these categories:

• waifu
• neko
• shinobu
• megumin
• bully
• cuddle
• cry
• hug
• awoo
• kiss
• lick
• pat
• smug
• bonk
• yeet
• blush
• smile
• wave
• highfive
• handhold
• nom
• bite
• glomp
• slap
• kill
• kick
• happy
• wink
• poke
• dance
• cringe"#,
            )
            .await
            .unwrap()
        }
    }
}
