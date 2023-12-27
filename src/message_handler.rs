use std::env;

use serenity::Error;
use serenity::{
    all::{ChannelId, Http},
    builder,
    client::Context,
    model::channel::Message,
};
use lazy_static::lazy_static;

lazy_static! {
 
    static ref EXCLUDED_ROLES: Vec<u64> = env::var("EXCLUDED_ROLES")
        .expect("expected `FREETHINKER_ROLE_IDS` to be set")
        .split(',')
        .map(|s| s
            .parse::<u64>()
            .expect("Each item in FREETHINKER_ROLE_IDS must be a valid u64"))
        .collect();

    static ref DO_NOT_PING: Vec<u64> = env::var("DO_NOT_PING")
        .expect("expected `FREETHINKER_ROLE_IDS` to be set")
        .split(',')
        .map(|s| s
            .parse::<u64>()
            .expect("Each item in FREETHINKER_ROLE_IDS must be a valid u64"))
        .collect();

    static ref GUILD_ID: u64 = env::var("DISCORD_GUILD_ID")
        .expect("expected `DISCORD_GUILD_ID` to be set")
        .parse()
        .expect("expected `DISCORD_GUILD_ID` to be a valid guild ID");
}


pub async fn message(ctx: Context, msg: Message) {
    // Listnen if the message is a command
    if msg.content.starts_with('!') {
        // Create slash commands, sends the result back as a string.
        match msg.content.as_str() {
            "!rule1" => {
                send_message(
                    &ctx.http,
                    &msg.channel_id,
                    "Keep conversation topics within the related channels provided.",
                )
                .await;
            }

            "!rule2" => {
                send_message(
                    &ctx.http,
                    &msg.channel_id,
                    "If you have a question about something that people do not know about in general open a ticket. <#${820850105552207872}>",
                )
                .await;
            }

            "!rule3" => {
                send_message(
                    &ctx.http,
                    &msg.channel_id,
                    "No explicit posts about politics, racism, suicide, mental illnesses, other forms of disabilities, targeted harassment, violence, drugs, sex, underage individuals, death, gore, etc. Please use discretion where appropriate.",
                )
                .await;
            }

            "!rule4" => {
                send_message(
                    &ctx.http,
                    &msg.channel_id,
                    "No impersonation, doxing, or revealing of unwanted personal information allowed, if possible, ask for consent to post a picture that has another user in the picture.",
                )
                .await;
            }

            "!rule5" => {
                send_message(&ctx.http, &msg.channel_id, "No explicit NSFW content.").await;
            }

            "!rule6" => {
                send_message(
                    &ctx.http,
                    &msg.channel_id,
                    "No offensive or rule breaking names and profiles.",
                )
                .await;
            }

            "!rule7" => {
                send_message(
                    &ctx.http,
                    &msg.channel_id,
                    "Do not troll, be toxic, or have heated arguments. Take them to private messages or another place outside of this server.
                    ",
                )
                .await;
            }

            "!rule8" => {
                send_message(
                    &ctx.http,
                    &msg.channel_id,
                    "No spamming or excessive pinging of users/moderators.",
                )
                .await;
            }

            "!rule9" => {
                send_message(
                    &ctx.http,
                    &msg.channel_id,
                    "No advertisements or promoting any products.",
                )
                .await;
            }

            "!rule10" => {
                send_message(
                    &ctx.http,
                    &msg.channel_id,
                    "Use common sense. Moderation team acts at their own discretion, and has the final say.",
                )
                .await;
            }

            "!rule11" => {
                send_message(
                    &ctx.http,
                    &msg.channel_id,
                    "Do not ask for any of the roles.",
                )
                .await;
            }

            "!rule12" => {
                send_message(
                    &ctx.http,
                    &msg.channel_id,
                    "DO NOT PING Finalboy or Finalmouse Employees.",
                )
                .await;
            }

            "!rule13" => {
                send_message(
                    &ctx.http,
                    &msg.channel_id,
                    "No reselling/second hand buying in this server. Use appropriate marketplaces for that.",
                )
                .await;
            }

            "!rule14" => {
                send_message(
                    &ctx.http,
                    &msg.channel_id,
                    "For all Finalmouse order inquiries, send an e-mail to support@finalmouse.com.",
                )
                .await;
            }

            // borger
            "!🍔" => {
                send_message(&ctx.http, &msg.channel_id, "🍔").await;
            }

            _ => {}
        }
    }

    // Check if the message mentions a user. Might want to create it's own handler for it. 
    if !msg.author.bot && !msg.mentions.is_empty() {
        // Fetch the Member object associated with the Message
        let guild_id = msg.guild_id.expect("Message must be from a guild");
        let member = guild_id
            .member(&ctx.http, msg.author.id)
            .await
            .expect("Failed to get member");

        // Check if the member has any of the excluded roles
        let has_excluded_role = member
            .roles
            .iter()
            .any(|role| EXCLUDED_ROLES.contains(&role.get()));
        if has_excluded_role {
            return;
        }

        // Check if the message mentions a user with a role in DO_NOT_PING
        for mention in &msg.mentions {
            let mentioned_member = guild_id
                .member(&ctx.http, mention.id)
                .await
                .expect("Failed to get mentioned member");
            let mentions_do_not_ping = mentioned_member
                .roles
                .iter()
                .any(|role| DO_NOT_PING.contains(&role.get()));
            if mentions_do_not_ping {
                send_message(
                    &ctx.http,
                    &msg.channel_id,
                    "Do not ping Finalboy or Finalmouse Staff",
                )
                .await;
                break;
            }
        }
    }

}

// contruct commands that respond to messages
async fn send_message(http: &Http, channel_id: &ChannelId, message: &str) {
    match channel_id.say(http, message).await {
        Ok(_) => {
            // Message sent successfully, no further action needed
        }
        Err(why) => {
            println!("Error sending message: {:?}", why);
        }
    }
}

// send message as the bot.
pub async fn post_message(http: &Http, channel_id: &ChannelId, message: &str) -> Result<(), Error> {
    match channel_id.say(http, message).await {
        Ok(_) => Ok(()),
        Err(why) => Err(why),
    }
}

// edit a message sent by the bot.
pub async fn edit_message(
    http: &Http,
    channel_id: &ChannelId,
    message_id: u64,
    edited_message: builder::EditMessage,
) -> Result<(), Error> {
    match channel_id
        .edit_message(http, message_id, edited_message)
        .await
    {
        Ok(_) => Ok(()),
        Err(why) => Err(why),
    }
}
