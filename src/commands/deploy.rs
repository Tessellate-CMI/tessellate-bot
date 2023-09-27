use serenity::{
  builder::CreateApplicationCommand,
  model::prelude::{
    application_command::ApplicationCommandInteraction, InteractionResponseType,
  },
  prelude::Context,
  utils::Color,
};

use super::HandleError;
use crate::ConfigData;

pub const NAME: &str = "deploy";
const DESCRIPTION: &str = "Deploy frontend to Cloudflare";

pub fn register(
  command: &mut CreateApplicationCommand,
) -> &mut CreateApplicationCommand {
  command.name(NAME).description(DESCRIPTION)
}

pub async fn run(
  ctx: &Context,
  command: &ApplicationCommandInteraction,
) -> Result<(), HandleError> {
  let data = ctx.data.read().await;
  let config = data.get::<ConfigData>().unwrap();

  let resp = config.cloudflare.create_deployment().await?;

  command
    .create_interaction_response(&ctx.http, |response| {
      response
        .kind(InteractionResponseType::ChannelMessageWithSource)
        .interaction_response_data(|message| {
          message.embed(|embed| {
            if resp.success {
              embed
                .title("Deployment request sent!")
                .url(resp.result.url)
                .color(Color::BLUE)
                .field("ID", resp.result.id.to_string(), false)
            } else {
              embed
                .title("Deployment request failed!")
                .url(resp.result.url)
                .color(Color::RED)
                .fields([
                  ("ID", resp.result.id.to_string(), false),
                  ("Error", resp.errors.to_string(), false),
                ])
            }
          })
        })
    })
    .await?;

  Ok(())
}
