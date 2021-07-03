// Imports
use teloxide::prelude::*;

use crate::{
    utils::get_image, Cxt
};

pub async fn gurl_command(cx: &Cxt, category: String) -> Message {
    let data = get_image(&category).await;

    match data {
        Ok(image) => cx
            .answer_photo(teloxide::types::InputFile::Url(image.url))
            .await
            .unwrap(),
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
