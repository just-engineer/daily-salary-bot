mod telegram_api;
mod scheduler;

use std::env;
use telegram_bot::*;
use crate::telegram_api::polling;

#[tokio::main]
async fn main() -> Result<(), Error>  {
    let token = env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not set");
    let event_time = env::var("MOS_TIME_EVENT").expect("MOS_TIME_EVENT not set");
    let updates_future = polling(token);
    updates_future.await

}