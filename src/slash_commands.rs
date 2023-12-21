#![allow(dead_code, unused_variables, unused_imports, private_interfaces)]
use std::{env, fs::File, vec};

use crate::{
    message_handler::{edit_message, post_message},
    slash_commands::process_token::Tokens,
};
use serenity::{
    all::{
        async_trait, ChannelId, Client, CommandInteraction, Context, CreateInteractionResponse,
        CreateInteractionResponseMessage, EventHandler, GatewayIntents, GuildId, Interaction,
        InteractionId, MessageId, UserId,
    },
    builder::{EditMessage, GetMessages},
    http,
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
            Self::Tweet(tweet) => tweet.run(command_info, ctx).await,
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
    MessageId {
        /// Amount of discord messages to send as a tweet
        message_id: String,
    },

    /// Sends all discord messages sent in the past specified minutes as a tweet
    Message {
        /// Amount of minutes
        message: String,
    },
}

impl TweetCommand {
    /// The `run` function takes a `TweetCommand` and a `command_info` and executes the command.
    /// It matches on the `TweetCommand` to determine what to do.
    async fn run(self, command_info: &CommandInteraction, ctx: &Context) -> String {
        match self {
            Self::MessageId { message_id } => tweet_message_id(message_id, command_info, ctx).await,
            Self::Message { message } => tweet_message(message, command_info).await,
        }
    }
}

/// The `tweet_message_id` function retrieves the messages from a channel, converts them to a vector of strings,
/// and prints each string on a new line.
/// It returns a string "Done" upon completion.
async fn tweet_message_id(
    message_id: String,
    command_info: &CommandInteraction,
    ctx: &Context,
) -> String {
    let user_id = command_info.user.id;
    let message_id_int = message_id.parse::<u64>();

    if message_id_int.is_err() {
        // If parsing fails, return the message_id as string
        return format!("'{}' is an invalid integer", message_id.clone());
    }

    // get all the messages sent since the given message_id:
    let channel_id = ChannelId::new(command_info.channel_id.into());
    // <TODO> If the bot does not have access to the channel, handle that

    let builder = GetMessages::new()
        .after(MessageId::new(message_id_int.unwrap()))
        .limit(100);
    // <TODO> If the message id is invalid, handle that

    let _messages = channel_id.messages(&ctx.http, builder).await;
    // <TODO> If this doesn't work for some reason, handle that

    // gets the message content, and divides it into strings with a max length of 280 characters
    let vec_of_messages = messages_to_string_vec(ctx, channel_id, builder, user_id).await;

    for message in &vec_of_messages {
        println!("{}", message);
    }

    String::from("Done")
}

/// The `messages_to_string_vec` function retrieves messages from a channel
/// and converts them to a vector of strings.
/// It only saves the messages sent by the user specified by `user_id`.
/// It goes through the messages backwards to stay in chronological order.
/// Once the total length of the messages exceeds 280 characters, it starts a new string.
async fn messages_to_string_vec(
    ctx: &Context,
    channel_id: ChannelId,
    builder: GetMessages,
    user_id: UserId,
) -> Vec<String> {
    let messages = channel_id.messages(&ctx.http, builder).await.unwrap();
    let mut message_strings = vec![String::new()];
    let mut current_string = 0;

    // Need to go through the messages backwards to stay in chronological order
    for message in messages.iter().rev() {
        // Check if the author of the message is the user we're interested in
        if message.author.id != user_id {
            continue;
        }

        let content = &message.content;
        if message_strings[current_string].len() + content.len() > 280 {
            // Start a new string if adding the next line makes it over 280 characters
            message_strings.push(String::new());
            current_string += 1;
        }

        // Add the message to the current string, followed by a space
        message_strings[current_string].push_str(content);
        message_strings[current_string].push(' ');
    }

    message_strings
}

async fn tweet_message(message: String, command_info: &CommandInteraction) -> String {
    let user_id = command_info.user.id;

    let all_messages: String = "sdf".to_string();

    format!(
        "All the messages you sent in the past {} minutes, I will tweet",
        message
    )
}
