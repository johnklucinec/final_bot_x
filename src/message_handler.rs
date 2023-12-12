use serenity::{
    all::{Http, ChannelId},
    client::Context,
    model::channel::Message
  };

pub async fn message(ctx: Context, msg: Message) {

    // Listnen if the message is a command
    if msg.content.starts_with('!') {

        // Create slash commands, sends the result back as a string. 
        match msg.content.as_str() {
            "!wintah" => {
                send_message(&ctx.http, &msg.channel_id, "Yes, Wintah does indeed like Overwatch!").await;
            }
    
            "!burger" => {
                send_message(&ctx.http, &msg.channel_id, "🍔").await;
            }
            
            _ => {}
        }

    }

    // Example to scan the contents of every message for a certain word. Excludes bot from scan. 
    if msg.author.id != ctx.cache.current_user().id {
        if msg.content.as_str().contains("rancho") {
            send_message(&ctx.http, &msg.channel_id, "Who is Rancho?").await;
        }
     }

     // Scan the messages of a certain user, and respond with String.
    match msg.author.name.as_str() {
        "racho" => {
            if let Err(why) = msg
                .channel_id
                .say(&ctx.http, "Ok Rachel")
                .await
            {
                println!("Error sending message: {:?}", why);
            }
        }
        _ => {}
    }

}

// contruct commands that respond to messages
async fn send_message(http: &Http, channel_id: &ChannelId, message: &str) {
    if let Err(why) = channel_id.say(http, message).await {
        println!("Error sending message: {:?}", why);
    }
}