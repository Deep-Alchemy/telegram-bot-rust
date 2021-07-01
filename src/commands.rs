// Define modules
mod gurl;
mod random;

// Imports
use std::error::Error;

use teloxide::{
    prelude::*,
    utils::command::BotCommand
};

// Define the main command handler
#[derive(BotCommand)]
#[command(rename = "lowercase", description = "These are the commands supported!")]
pub enum Command {
    #[command(description = "Get Help")]
    Help,
    #[command(description = "Get from random category")]
    Random,
    #[command(description = "Get from particular category.")]
    Gurl(String),
}

// Command handler function
pub async fn handle_commands(
    cx: UpdateWithCx<AutoSend<Bot>, Message>,
    command: Command,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    // Match commands
    match command {
        Command::Help => cx.answer(Command::descriptions()).await.unwrap(),
        Command::Random => random::random_command(cx).await,
        Command::Gurl(category) => gurl::gurl_command(cx, category).await
    };

    // Return OK
    Ok(())
}
