use crate::slash_commands::AllCommands;
use serenity::{
    all::Interaction,
    builder::{CreateInteractionResponse, CreateInteractionResponseMessage},
    client::Context,
};
use serenity_commands::Commands;

pub async fn interaction_create(ctx: Context, interaction: Interaction) {
    if let Interaction::Command(command) = interaction {
        let command_data = AllCommands::from_command_data(&command.data).unwrap();
        command
            .create_response(
                &ctx,
                CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new()
                        .content(command_data.run(&command, &ctx).await)
                        .ephemeral(true),
                ),
            )
            .await
            .unwrap();
    }
}
