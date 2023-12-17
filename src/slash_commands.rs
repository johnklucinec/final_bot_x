#![allow(dead_code, unused_variables, unused_imports, private_interfaces)]
use std::fs::File;

use serenity::{all::{
    async_trait, Client, Context, CreateInteractionResponse, CreateInteractionResponseMessage,
    EventHandler, GatewayIntents, GuildId, Interaction, CommandInteraction, InteractionId,
}, utils::MessageBuilder};
use serenity_commands::{Command, Commands, SubCommand};
use crate::{message_handler::post_message, slash_commands::process_token::Tokens};

#[derive(Debug, Commands)]
pub enum AllCommands {
    /// Ping the bot.
    Ping,

    /// Echo a message.
    Post {
        /// The message to echo.
        message: String,
        // The attachment?
        //attatchment: File,
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
            Self::Post { message} => post(message, &command_info, &ctx).await,
            Self::Latest => get_latest().await,
            Self::Tweet(tweet) => tweet.run(),
            Self::Register { token } => register_user(token, &command_info).await,
        }
    }
}

mod get_latest_tweet;
mod process_token;

async fn get_latest() -> String {
    use get_latest_tweet::get_latest_tweet;

    match get_latest_tweet().await {
        Ok(tweet_link) => tweet_link,
        Err(_) => "Error calling rettiwt-api".to_owned(),
    }
}

async fn post(message: String, command_info: &CommandInteraction, ctx: &Context) -> String {

    match post_message(&ctx.http, &command_info.channel_id, &message).await {
        Ok(_) => {
            String::from("Done!")
        },
        Err(why) => {
            format!("Error sending message: {:?}", why)
        }
    }
}

async fn register_user(token: String, command_info: &CommandInteraction) -> String {
    let user_id = command_info.user.id.to_string();
    let channel = command_info.channel_id;

    let mut tokens = Tokens::load().unwrap_or_default();

    match tokens.find_token_by_user_id(&user_id) {
        None => {
            tokens.add_token(user_id.clone(), token.clone());
            format!("We have successfully registered your token, <@{}>", user_id)
        }
        Some(_) => {
            tokens.add_token(user_id.clone(), token.clone());
            format!("Your Twitter token has been updated, <@{}>", user_id)
        }
    }
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
