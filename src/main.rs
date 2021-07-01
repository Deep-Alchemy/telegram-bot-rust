// Define modules
mod commands;
mod config;
mod models;
mod utils;

// Imports
use std::env;

use teloxide::prelude::*;

// Define custom types
pub type Cxt = UpdateWithCx<AutoSend<Bot>, Message>;
pub type Ctx = UpdateWithCx<AutoSend<Bot>, CallbackQuery>;

#[tokio::main]
async fn main() {
    run().await
}

async fn run() {
    // Enable teleoxide logging
    teloxide::enable_logging!();

    // Log start
    log::info!("Logging in as {}", config::BOT_NAME);

    // Login with a bot token from the environment
    let token = env::var("BOT_TOKEN").expect("`BOT_TOKEN` Missing! Please initialize it.");

    // Initialize bot
    let bot = Bot::new(token).auto_send();

    // Init commands
    teloxide::commands_repl(bot, config::BOT_NAME, commands::handle_commands)
        .await
}
