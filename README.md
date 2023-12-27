# FinalBotX

A multi-use discord bot made in Rust!

## Features
  * Made with Rust (fast)
  * Custom server automod
  * Channel specific moderation
  * Get the latest tweets from Finalmouse
  * Send tweets from Discord to personal Twitter
  * Add roles to users


## Supported commands and features

### Slash Commands
  * /latest - Gets the latest tweet from Finalmouse. 
    * Gets the latest tweet from the Finalmouse twitter, applys fx fix, pings twitter role, and posts the tweet. 
    * Currently, if the latest tweet is a retweet, it will grab that.
  * /tweet ```message```
    * tweets the message that was typed into the command.
    * user has to register their twitter token before using this command.
  * /tweet ```message_id```
    * post all the messages including and after the given message.
    * automatically breaks discord messages into strings less then 280 characters (twitters character limit).
    * user has to register their twitter token before using this command.
  * /register ```twitter_token```
    * allows a user to register thier twitter token.
    * automatically saved so user does not have to register each time they want to tweet or if the bot restarts
  * /post ```message```
    * post something on behalf of the discord bot
  * /edit ```message_id```
    * edit any message sent by the bot. This is helpful when you have a channel that you want to keep clutter free. It allows multiple people to edit the same message.
  * /ping 
    * test to see if the bot works 
### Message Commands (listeners) 
  * !rule1, rule2... 
    * The bot sends the corresponding rule in the channel where this command was typed
  * Posts a message if a user pings Finalboy or Finalmouse Staff
    * This rule does not apply to users that have a role in the EXCLUDED_ROLE env.
  * Automatically gives user a role if their image is approved.
    * Only gives roles that are in the FREETHINKER_ROLE_IDS env.
    * Gives a new role when the ✅ emoji is given.
    * DM's the user when the ❌ is given with the reason why.



## Example
Coming Soon

## Setup
Coming Soon

## Future Versions/Features
  * Encrypt user tokens
  * Fully implement the ability to post tweets
    * The bot currently does not post the tweets, as it does not fully work with OAuth.
    * Have it work for images as well.
    * All the other features are implemented
  * Better /post and /edit commands
    * It would be cool if the bot generated a link where the user could preview the message they were creating or editing.
    * Here is an example: https://autocode.com/tools/discord/embed-builder/
  * Make the commands mod only. This should be coming in a future serenity_commands update.
## Questions/Suggestions/Bug Reports
Add me on Discord: DaddyJuanito
