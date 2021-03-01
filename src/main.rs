mod telegram_api;

use std::env;
use telegram_bot::*;
use crate::telegram_api::polling;

#[tokio::main]
async fn main() -> Result<(), Error>  {
    let token = env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not set");
    polling(token).await
}