use std::env;
use lazy_static::lazy_static;
use serenity::http::Http;
use serenity::model::id::RoleId;
use serenity::{all::Reaction, client::Context};

lazy_static! {
    static ref FREETHINKER_TRIALS: u64 = env::var("FREETHINKER_TRIALS")
        .expect("expected `FREETHINKER_TRIALS` to be set")
        .parse()
        .expect("FREETHINKER_TRIALS must be a valid u64");

    static ref GUILD_ID: u64 = env::var("DISCORD_GUILD_ID")
        .expect("expected `DISCORD_GUILD_ID` to be set")
        .parse()
        .expect("expected `DISCORD_GUILD_ID` to be a valid guild ID");

    static ref FREETHINKER_ROLE_IDS: Vec<u64> = env::var("FREETHINKER_ROLE_IDS")
        .expect("expected `FREETHINKER_ROLE_IDS` to be set")
        .split(',')
        .map(|s| s
            .parse::<u64>()
            .expect("Each item in FREETHINKER_ROLE_IDS must be a valid u64"))
        .collect();
}

pub async fn reaction_add(ctx: Context, add_reaction: Reaction) {
    // Check if the reaction is in the channel of interest
    if add_reaction.channel_id.to_string() == *FREETHINKER_TRIALS.to_string() {
        // Get the users who reacted with the same emoji
        let users = add_reaction
            .users(
                &ctx.http,
                add_reaction.emoji.clone(),
                Some(2),
                None::<serenity::all::UserId>,
            )
            .await
            .unwrap_or_default();

        // If more than one user reacted, do nothing
        if users.len() > 1 {
            return;
        }

        // Check if the reaction is one of the three roles
        match add_reaction.emoji.to_string().as_str() {
            "✅" => {
                next_role(&ctx.http, &add_reaction).await;
            }
            "❌" => {
                dm_user(&ctx, &add_reaction).await;
            }
            _ => {}
        }
    }
}

// gives the user the next finalmouse role
// if do not have a role, it will give them the Freethinker role
// if they have Starking, this function will do nothing
async fn next_role(http: &Http, add_reaction: &Reaction) {
    let guild_id = add_reaction.guild_id.unwrap();
    let channel_id = add_reaction.channel_id;
    let message = channel_id
        .message(&http, add_reaction.message_id)
        .await
        .unwrap();
    let user_id = message.author.id;

    // Fetch the member
    let member_result = guild_id.member(&http, user_id).await;

    if let Ok(member) = member_result {
        // Get the current roles of the member
        let current_roles = &member.roles;

        // Find the next role to be assigned from the FREETHINKER_ROLE_IDS vector
        let next_role = FREETHINKER_ROLE_IDS
            .iter()
            .find(|&&role_id| !current_roles.contains(&RoleId::new(role_id)));

        // If there is a next role, add it to the member
        if let Some(&role_id) = next_role {
            if let Err(why) = member.add_role(&http, role_id).await {
                println!("Error adding role: {:?}", why);
            }
        }
    } else {
        println!("Could not fetch member");
    }
}

// sends the user a dm if their photo does not get approved. 
async fn dm_user(ctx: &Context, add_reaction: &Reaction) {
    let message = add_reaction
        .channel_id
        .message(&ctx.http, add_reaction.message_id)
        .await
        .unwrap();
    let user_id = message.author.id;

    if let Ok(dm_channel) = user_id.create_dm_channel(&ctx.http).await {
        let error_message = format!(
            "Sorry, your photo in {} was not approved.",
            message.link()
        );
        let _ = dm_channel.say(&ctx.http, error_message).await;
    }
}
