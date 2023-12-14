#![allow(dead_code, unused_variables, unused_imports, private_interfaces)]
use serenity::all::{
    async_trait, Client, Context, CreateInteractionResponse, CreateInteractionResponseMessage,
    EventHandler, GatewayIntents, GuildId, Interaction,
};
use serenity_commands::{Command, Commands, SubCommand};

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
}

impl AllCommands {
    pub async fn run(self) -> String {
        match self {
            Self::Ping => "Pong!".to_string(),
            Self::Echo { message } => message,
            Self::Latest => {get_latest().await},
            Self::Tweet(tweet) => tweet.run(),
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
