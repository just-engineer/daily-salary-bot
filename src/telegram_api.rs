use futures::StreamExt;
use telegram_bot::*;

pub async fn polling(token: String) -> Result<(), Error> {
    let api = Api::new(token);

    // Fetch new updates via long poll method
    let mut stream = api.stream();
    while let Some(update) = stream.next().await {
        // If the received update contains a new message...
        let update = update?;
        if let UpdateKind::Message(message) = update.kind {
            if let MessageKind::Text { ref data, .. } = message.kind {
                println!("<{}>: {}", &message.from.first_name, data);

                // Answer message with "Hi".
                let string_message = format!(
                    "Hi, {}! Sorry I can't do nothing with your message",
                    &message.from.first_name);
                api.send(message.text_reply(string_message))
                    .await?;
            }
        }
    }
    Ok(())
}
