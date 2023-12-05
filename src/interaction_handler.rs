use serenity::{
  client::{Context},
  builder::{CreateInteractionResponse, CreateInteractionResponseMessage},
  all::Interaction,
};
use crate::slash_commands::Commands;
use serenity_commands::CommandData;

pub async fn interaction_create(ctx: Context, interaction: Interaction) {
  if let Interaction::Command(command) = interaction {
      let command_data = Commands::from_command_data(&command.data).unwrap();
      command
          .create_response(
              ctx,
              CreateInteractionResponse::Message(
                  CreateInteractionResponseMessage::new()
                      .content(format!("```rs\n{command_data:?}```")),
              ),
          )
          .await
          .unwrap();
  }
}