use serenity::{
    all::{GatewayIntents, GuildId, Interaction},
    async_trait,
    model::channel::Message,
    client::{Context, EventHandler},
    Client,
};
use serenity_commands::CommandData;
use std::env;
use dotenv::dotenv;
use slash_commands::Commands;

mod slash_commands;
mod interaction_handler;
mod message_handler;

struct Handler {
    guild_id: GuildId,
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, _: serenity::model::gateway::Ready) {
        self.guild_id
            .set_commands(&ctx, Commands::to_command_data())
            .await
            .unwrap();
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        interaction_handler::interaction_create(ctx, interaction).await;
    }

    async fn message(&self, ctx: Context, msg: Message) {
        message_handler::message(ctx, msg).await;
    }

}

#[tokio::main]
pub async fn main() {
    dotenv().ok();
    let token = env::var("DISCORD_TOKEN").expect("expected `DISCORD_TOKEN` to be set");

    let guild_id = env::var("DISCORD_GUILD_ID")
        .expect("expected `DISCORD_GUILD_ID` to be set")
        .parse()
        .expect("expected `DISCORD_GUILD_ID` to be a valid guild ID");

        let mut client = Client::builder(token, GatewayIntents::all())
        .event_handler(Handler { guild_id })
        .await
        .expect("client should be created successfully");

    client
        .start()
        .await
        .expect("client should start successfully");
}

