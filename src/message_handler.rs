use serenity::Error;
use serenity::{
    all::{ChannelId, Http},
    builder,
    client::Context,
    model::channel::Message,
};

pub async fn message(ctx: Context, msg: Message) {
    // Listnen if the message is a command
    if msg.content.starts_with('!') {
        // Create slash commands, sends the result back as a string.
        match msg.content.as_str() {
            "!wintah" => {
                send_message(
                    &ctx.http,
                    &msg.channel_id,
                    "I am now working on my discord bot bozo!
                    Get Better",
                )
                .await;
            }

            "!burger" => {
                send_message(&ctx.http, &msg.channel_id, "🍔").await;
            }

            _ => {}
        }
    }

    // Example to scan the contents of every message for a certain word. Excludes bot from scan.
    if msg.author.id != ctx.cache.current_user().id && msg.content.as_str().contains("rancho") {
        send_message(&ctx.http, &msg.channel_id, "Who is Rancho?").await;
    }

    // Scan the messages of a certain user, and respond with String.
    match msg.author.name.as_str() {
        "racho" => {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Ok Rachel").await {
                println!("Error sending message: {:?}", why);
            }
        }
        _ => {}
    }
}

// contruct commands that respond to messages
async fn send_message(http: &Http, channel_id: &ChannelId, message: &str) {
    match channel_id.say(http, message).await {
        Ok(_) => {
            // Message sent successfully, no further action needed
        }
        Err(why) => {
            println!("Error sending message: {:?}", why);
        }
    }
}

// send message as the bot.
pub async fn post_message(http: &Http, channel_id: &ChannelId, message: &str) -> Result<(), Error> {
    match channel_id.say(http, message).await {
        Ok(_) => Ok(()),
        Err(why) => Err(why),
    }
}

// edit a message sent by the bot.
pub async fn edit_message(
    http: &Http,
    channel_id: &ChannelId,
    message_id: u64,
    edited_message: builder::EditMessage,
) -> Result<(), Error> {
    match channel_id
        .edit_message(http, message_id, edited_message)
        .await
    {
        Ok(_) => Ok(()),
        Err(why) => Err(why),
    }
}
