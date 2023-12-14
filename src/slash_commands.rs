#![allow(dead_code, unused_variables, unused_imports, private_interfaces)]
use serenity::all::{
    async_trait, Client, Context, CreateInteractionResponse, CreateInteractionResponseMessage,
    EventHandler, GatewayIntents, GuildId, Interaction, CommandInteraction, InteractionId,
};
use serenity_commands::{Command, Commands, SubCommand};
use crate::message_handler::send_message;

#[derive(Debug, Commands)]
pub enum AllCommands {
    /// Ping the bot.
    Ping,

    /// Echo a message.
    Echo {
        /// The message to echo.
        message: String,
    },

    /// Get the latest tweet
    Latest,

    /// Set a tweet
    Tweet(TweetCommand),

    /// Register twitter token
    Register {
        /// twitter token
        token: String,
    },
}

impl AllCommands {
    pub async fn run(self, command_info: &CommandInteraction, ctx: &Context) -> String {

        match self {
            Self::Ping => "Pong!".to_string(),
            Self::Echo { message } => message,
            Self::Latest => get_latest().await,
            Self::Tweet(tweet) => tweet.run(),
            Self::Register { token } => register_user(token, &command_info, &ctx).await,
        }
    }
}

mod get_latest_tweet;

async fn get_latest() -> String {
    use get_latest_tweet::get_latest_tweet;

    match get_latest_tweet().await {
        Ok(tweet_link) => tweet_link,
        Err(_) => "Error calling rettiwt-api".to_owned(),
    }
}

async fn register_user(token: String, command_info: &CommandInteraction, ctx: &Context) -> String {

    let user_id = command_info.id.to_string();
    let channel = command_info.channel_id;

    send_message(&ctx.http, &command_info.channel_id, "🍔").await;

    
    format!("We have succesfully registered your token {}, {}", user_id, token)
}


#[derive(Debug, Command)]
enum TweetCommand {
    /// Sends the past specified messages as a tweet
    Past {
        /// Amount of discord messages to send as a tweet
        messages: u64,
    },

    /// Sends all discord messages sent in the past specified minutes as a tweet
    Time {
        /// Amount of minutes
        minutes: u64,
    },

    /// Sends all the discord messages sent since (inclusive) specified discord message id
    Since {
        /// discord message id
        message_id: u64,
    },
}

impl TweetCommand {
    fn run(self) -> String {
        match self {
            Self::Past { messages } => format!("I will tweet the past {} discord messages", messages),
            Self::Time { minutes } => format!("All the messages you sent in the past {} minutes, I will tweet", minutes),
            Self::Since { message_id } => format!("I will tweet all the messages since {}", message_id),
        }
    }
}
