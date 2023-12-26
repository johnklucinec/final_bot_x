use serenity::{all::{CommandInteraction, ChannelId, MessageId, UserId}, client::Context, builder::GetMessages};
use super::{process_token::Tokens, post_tweet};

/// The `tweet_message_id` function retrieves the messages from a channel, converts them to a vector of strings,
/// and prints each string on a new line.
pub async fn tweet_message_id(
  message_id: String,
  command_info: &CommandInteraction,
  ctx: &Context,
) -> String {
  let user_id = command_info.user.id;
  let user_id_string = command_info.user.id.to_string();

  let user_token = match Tokens::load()
      .unwrap_or_default()
      .find_token_by_user_id(&user_id_string)
  {
      None => {
          return "Please use /register to register your Twitter token before using this command.".to_string();
      }
      Some(token) => token,
  };

  let message_id_int = match message_id.parse::<u64>() {
      Ok(value) => value,
      Err(_) => {
          // If parsing fails, return an informative error message
          return format!("'{}' is an invalid integer", message_id);
      }
  };

  // get all the messages sent since the given message_id:
  let channel_id = ChannelId::new(command_info.channel_id.into());
  let new_message_id = MessageId::new(message_id_int);

  // only gets the messages *after* the given message id
  let builder = GetMessages::new().after(new_message_id).limit(100);
  let _messages = channel_id.messages(&ctx.http, builder).await;

  if _messages.is_err() {
      return String::from(
          "Error getting the messages. Does the bot have access to this channel?",
      );
  }

  // gets the message content, and divides it into strings with a max length of 280 characters
  let vec_of_messages = messages_to_string_vec(ctx, channel_id, builder, user_id).await;

  // make sure there are error messages for token error, user token error, and connection error.

  for message in &vec_of_messages {
      println!("{}", message);
  }

  let params_vec = vec_of_messages
      .into_iter()
      .map(|message| post_tweet::TweetParams {
          text: Some(message),
          media: None,
          reply: None,
      })
      .collect();

  match post_tweet::send_tweets(params_vec, &user_token).await {
      Ok(()) => ("This feature is still being developed").to_string(),
      Err(error) => {
          eprintln!("Error sending tweets: {}", error);
          ("There was an error posting the tweets").to_string()
      }
  }
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

/// The `tweet_message` function retrieves the messages from a command and posts it as a tweet
pub async fn tweet_message(message: String, command_info: &CommandInteraction) -> String {
  let tweet_params = post_tweet::TweetParams {
      text: Some(message),
      media: None,
      reply: None,
  };

  let user_id = command_info.user.id;
  let user_id_string = command_info.user.id.to_string();

  let user_token = match Tokens::load()
      .unwrap_or_default()
      .find_token_by_user_id(&user_id_string)
  {
      None => {
          return "Please use /register to register your Twitter token before using this command.".to_string();
      }
      Some(token) => token,
  };

  match post_tweet::send_tweet(tweet_params, &user_token).await {
      Ok(_) => "This feature is still being developed".to_string(),
      Err(error) => {
          eprintln!("Error sending tweets: {}", error);
          "There was an error posting the tweets".to_string()
      }
  }
}
