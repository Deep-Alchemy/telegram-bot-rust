use teloxide::{
    payloads::SendMessageSetters,
    prelude::Message,
    types::{
        InlineKeyboardButtonKind,
        InlineKeyboardMarkup,
        InlineKeyboardButton
    }
};

use crate::Cxt;

pub async fn help_command(cx: &Cxt, description: String) -> Message {
    let github_url = "https://github.com/Deep-Alchemy/telegram-bot-rust".to_string();

    let github_button = InlineKeyboardButton::new("GitHub", InlineKeyboardButtonKind::Url(github_url));

    let keyboard = InlineKeyboardMarkup::default().append_row(vec![github_button]);

    cx.answer(description).reply_markup(keyboard).await.unwrap()
}
