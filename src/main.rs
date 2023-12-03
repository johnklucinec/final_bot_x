use std::env;

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    all::ClientBuilder,
    prelude::*
};

const WINTAH_GAY: &str = "Yes, Wintah does, in fact, like men.";

const WINTAH_COMMAND: &str = "!gay";

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == WINTAH_COMMAND {
            if let Err(why) = msg.channel_id.say(ctx.http, WINTAH_GAY).await {
                println!("Error sending message: {:?}", why);
        }
    }
}


async fn ready(&self, _: Context, ready: Ready) {
    println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    //dotenv::dotenv().ok();

    //let token = env::var("DISCORD_TOKEN")
    //.expect("Expected a token in the enviroment");

    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::DIRECT_MESSAGES;

    let mut client = ClientBuilder::new(&"MTE4MDMyMTM1MzgyODY3MTUwMA.GFR7t3.HcuJEZpzQG2QEJXYn9EhGDyGLA3bjjpRfOzxW8", intents)
    .event_handler(Handler)
    .await
    .expect("Err creating the client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why)
    }
}