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
            Self::Latest => {
                get_latest().await
            },
            Self::Tweet(tweet) => tweet.run(),
        }
    }
}

mod get_latest_tweet;

async fn get_latest() -> String {

    use get_latest_tweet::get_latest_tweet;

    get_latest_tweet().await
    
}

#[derive(Debug, Command)]
enum TweetCommand {
    /// Send the past a numbers as a tweet
    Past {
        /// Amount of discord messages to send as a tweet
        messages: f64,
    },

    /// Send all the tweets send in the past minute as a tweet
    Time {
        /// Amount of minutes
        minutes: f64,
    },
}

impl TweetCommand {
    fn run(self) -> String {
        match self {
            Self::Past { messages } => format!("I will tweet the past {} discord messages", messages),
            Self::Time { minutes } => format!("All the messages you sent in the past {} minutes, I will tweet", minutes),
        }
    }
}
