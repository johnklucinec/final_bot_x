use serenity::{
    client::Context,
    model::channel::Message
  };

pub async fn message(ctx: Context, msg: Message) {
    match msg.content.as_str() {
        "!wintah" => {
            if let Err(why) = msg
                .channel_id
                .say(&ctx.http, "Yes, Wintah does indeed like men 🥵")
                .await
            {
                println!("Error sending message: {:?}", why);
            }
        }
        _ => {}
    }

    match msg.author.name.as_str() {
        "tyethetoster" => {
            if let Err(why) = msg
                .channel_id
                .say(&ctx.http, "Nice balls bro!")
                .await
            {
                println!("Error sending message: {:?}", why);
            }
        }
        _ => {}
    }
}
