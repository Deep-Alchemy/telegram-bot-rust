// Define modules
mod gurl;
mod idea;
mod quote;
mod random;
mod advice;

// Imports
use std::error::Error;

use teloxide::{
    prelude::*, utils::command::BotCommand
};

// Define the main command handler
#[derive(BotCommand)]
#[command(
    rename = "lowercase",
    description = "These are the commands supported!"
)]
pub enum Command {
    #[command(description = "Get Help")]
    Help,
    #[command(description = "Get from random category")]
    Random,
    #[command(description = "Get from particular category.")]
    Gurl(String),
    #[command(description = "Get a random Anime Quote, or provide character name.")]
    Quote(String),
    #[command(description = "Get some piece of advice.")]
    Advice,
    #[command(description = "Get a random idea!")]
    Idea,
}

// Command handler function
pub async fn handle_commands(
    cx: UpdateWithCx<AutoSend<Bot>, Message>,
    command: Command,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    // Match commands
    match command {
        Command::Help => cx.answer(Command::descriptions()).await.unwrap(),
        Command::Random => random::random_command(&cx).await,
        Command::Gurl(category) => gurl::gurl_command(&cx, category).await,
        Command::Quote(character) => quote::quote_command(&cx, character).await,
        Command::Advice => advice::advice_command(&cx).await,
        Command::Idea => idea::idea_command(&cx).await,
    };

    // Return OK
    Ok(())
}
