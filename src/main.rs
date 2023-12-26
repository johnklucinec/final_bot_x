use dotenv::dotenv;
use serenity::{
    all::{GatewayIntents, GuildId, Interaction, Reaction},
    async_trait,
    client::{Context, EventHandler},
    model::channel::Message,
    Client,
};
use serenity_commands::Commands;
use slash_commands::AllCommands;
use std::env;
use lazy_static::lazy_static;

mod interaction_handler;
mod message_handler;
mod slash_commands;

struct Handler {
    guild_id: GuildId,
}

lazy_static! {
    static ref FREETHINKER_TRIALS: u64 = env::var("FREETHINKER_TRIALS")
        .expect("expected `FREETHINKER_TRIALS` to be set")
        .parse()
        .expect("FREETHINKER_TRIALS must be a valid u64");
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, _: serenity::model::gateway::Ready) {
        self.guild_id
            .set_commands(&ctx, AllCommands::create_commands())
            .await
            .unwrap();
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        interaction_handler::interaction_create(ctx, interaction).await;
    }

    async fn message(&self, ctx: Context, msg: Message) {
        message_handler::message(ctx, msg).await;
    }

    async fn reaction_add(&self, ctx: Context, add_reaction: Reaction) {
        // Check if the reaction is in the channel of interest
        if add_reaction.channel_id.to_string() == *FREETHINKER_TRIALS.to_string() {
            // Get the users who reacted with the same emoji
            let users = add_reaction
                .users(&ctx.http, add_reaction.emoji.clone(), Some(2), None::<serenity::all::UserId>)
                .await
                .unwrap_or_default();
 
            // If more than one user reacted, do nothing
            if users.len() > 1 {
                return;
            }
 
            // Check if the reaction is one of the three roles
            match add_reaction.emoji.to_string().as_str() {
                "✅" => {
                   println!("A check mark was added!")
                }
                "🍔" => {
                   println!("A burger was added!")
                }
                "🧑‍🦲" => {
                   println!("A person was added!")
                }
                _ => {}
            }
        }
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
