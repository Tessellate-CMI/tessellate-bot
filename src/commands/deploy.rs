use serenity::{
  builder::CreateApplicationCommand,
  model::prelude::{
    application_command::ApplicationCommandInteraction, InteractionResponseType,
  },
  prelude::Context,
  utils::Color,
};
use tracing::error;

use super::HandleError;
use crate::ConfigData;

pub const NAME: &str = "deploy";
const DESCRIPTION: &str = "Deploy frontend to Cloudflare Pages";

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

  for err in resp.errors {
    error!("Deployment error: {err:?}");
  }

  Ok(
    command
      .create_interaction_response(&ctx.http, |response| {
        response
          .kind(InteractionResponseType::ChannelMessageWithSource)
          .interaction_response_data(|message| {
            message.embed(|embed| {
              embed
                .title("Deployment!")
                .url(resp.result.url)
                .color(if resp.success {
                  Color::BLUE
                } else {
                  Color::RED
                })
                .fields([
                  ("ID", resp.result.id.to_string(), false),
                  ("Success", resp.success.to_string(), false),
                ])
            })
          })
      })
      .await?,
  )
}
