use teloxide::{prelude::*, utils::command::BotCommand};

mod utils;

use std::error::Error;
use utils::{get_image, get_random_image};
#[derive(BotCommand)]
#[command(
    rename = "lowercase",
    description = "These are the commands supported!"
)]
enum Command {
    #[command(description = "Get Help")]
    Help,
    #[command(description = "Get from random category")]
    Random,
    #[command(description = "Get from particular category.")]
    Gurl(String),
}

async fn reply(
    cx: UpdateWithCx<AutoSend<Bot>, Message>,
    command: Command,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    match command {
        Command::Help => cx.answer(Command::descriptions()).await.unwrap(),
        Command::Random => {
            let random_image = get_random_image().await;
            cx.answer_photo(teloxide::types::InputFile::Url(random_image.url))
                .await?
        }
        Command::Gurl(category) => {
            let image = get_image(&category).await;
            cx.answer_photo(teloxide::types::InputFile::Url(image.url))
                .await?
        }
    };
    Ok(())
}
#[tokio::main]
async fn main() {
    run().await
}

async fn run() {
    // Enable teleoxide logging
    teloxide::enable_logging!();

    // Log start
    log::info!("Starting BOT...");

    // Login with a bot token from the environment
    let token = std::env::var("BOT_TOKEN")
    .expect("`BOT_TOKEN` Missing! Please initialize it");

    // Initialize bot
    let bot = Bot::new(token).auto_send();

    // Init commands
    teloxide::commands_repl(bot, "Your Bot Name", reply).await
}
