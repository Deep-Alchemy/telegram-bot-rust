// Imports
use teloxide::prelude::*;

use crate::{
    utils::get_random_image,
    Cxt,
};

use super::gurl::send_image;

pub async fn random_command(cx: &Cxt) -> Message {
    let random_image = get_random_image().await;

    send_image(cx, random_image).await
 
}
