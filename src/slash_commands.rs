#![allow(dead_code, unused_variables, unused_imports, private_interfaces)]
use std::{env, fs::File};

use crate::{
    message_handler::{edit_message, post_message},
    slash_commands::process_token::Tokens,
};
use serenity::{
    all::{
        async_trait, Client, CommandInteraction, Context, CreateInteractionResponse,
        CreateInteractionResponseMessage, EventHandler, GatewayIntents, GuildId, Interaction,
        InteractionId,
    },
    builder::EditMessage,
    utils::MessageBuilder,
};
use serenity_commands::{Command, Commands, SubCommand};

mod post_tweet;

#[derive(Debug, Commands)]
pub enum AllCommands {
    /// Ping the bot.
    Ping,

    /// Echo a message.
    Post {
        /// The message to Post.
        message: String,
    },

    /// Edits a message sent by the bot with message_id
    Edit {
        /// The message_id
        message_id: String,
        /// The edited message
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
            Self::Post { message } => post(message, command_info, ctx).await,
            Self::Edit {
                message_id,
                message,
            } => edit(message, command_info, ctx, message_id).await,
            Self::Latest => get_latest(command_info, ctx).await,
            Self::Tweet(tweet) => tweet.run(command_info).await,
            Self::Register { token } => register_user(token, command_info).await,
        }
    }
}

mod get_latest_tweet;
mod process_token;

#[allow(unused_assignments)]
async fn get_latest(command_info: &CommandInteraction, ctx: &Context) -> String {
    use get_latest_tweet::get_latest_tweet;
    let discord_role = env::var("TWITTER_ROLE").expect("expected `TWITTER_ROLE` to be set");

    let mut tweet_link = String::new();

    match get_latest_tweet().await {
        Ok(link) => tweet_link = link,
        Err(err) => return format!("Error calling rettiwt-api: {}", err),
    }

    let message = format!("<@&{}>\n\n{}", discord_role, tweet_link);
    post(message, command_info, ctx).await
}

async fn post(message: String, command_info: &CommandInteraction, ctx: &Context) -> String {
    match post_message(&ctx.http, &command_info.channel_id, &message).await {
        Ok(_) => String::from("Done!"),
        Err(why) => {
            println!("{:?}", why);
            String::from(
                "Error editing message: Check bot console for more information

Does the bot have write access in the server and the channel?",
            )
        }
    }
}

async fn edit(
    message: String,
    command_info: &CommandInteraction,
    ctx: &Context,
    message_id: String,
) -> String {
    let content = EditMessage::new().content(message);

    let message_id_int = message_id.parse::<u64>();

    if message_id_int.is_err() {
        // If parsing fails, return the message_id as string
        return format!("'{}' is an invalid integer", message_id.clone());
    }

    match edit_message(
        &ctx.http,
        &command_info.channel_id,
        message_id_int.unwrap(),
        content,
    )
    .await
    {
        Ok(_) => String::from("Done!"),
        Err(why) => {
            println!("{:?}", why);
            String::from(
                "Error editing message: Check bot console for more information
                
Did you use the correct message id?",
            )
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
    Message {
        /// Amount of minutes
        message: String,
    },

    /// Sends all the discord messages sent since (inclusive) specified discord message id
    Since {
        /// discord message id
        message_id: String,
    },
}

impl TweetCommand {
    async fn run(self, command_info: &CommandInteraction) -> String {
        match self {
            Self::Past { messages } => {
                tweet_past_messages(messages, command_info).await
            },
            Self::Message { message } => {
                tweet_message(message, command_info).await
            },
            Self::Since { message_id } => {
                format!("I will tweet all the messages since {}", message_id)
            },
        }
    }
}

async fn tweet_past_messages(messages: u64, command_info: &CommandInteraction) -> String {
    let user_id = command_info.user.id;

    let all_messages: String = "sdf".to_string();

    format!("I will tweet the past {} discord messages", messages)
}

async fn tweet_message(message: String, command_info: &CommandInteraction) -> String {
    let user_id = command_info.user.id;

    let all_messages: String = "sdf".to_string();

    format!("All the messages you sent in the past {} minutes, I will tweet", message)
}