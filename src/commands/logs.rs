use std::borrow::Cow;

use serde_json::value::Value;
use serenity::{
  builder::CreateApplicationCommand,
  model::prelude::{
    application_command::ApplicationCommandInteraction,
    command::CommandOptionType, AttachmentType, InteractionResponseType,
  },
  prelude::Context,
};

use super::{respond_text, HandleError};
use crate::ConfigData;

pub const NAME: &str = "logs";
const DESCRIPTION: &str = "Fetch Cloudflare logs";

pub fn register(
  command: &mut CreateApplicationCommand,
) -> &mut CreateApplicationCommand {
  command
    .name(NAME)
    .description(DESCRIPTION)
    .create_option(|option| {
      option
        .kind(CommandOptionType::String)
        .required(true)
        .name("deployment_id")
        .description("Deployment ID")
    })
}

pub async fn run(
  ctx: &Context,
  command: &ApplicationCommandInteraction,
) -> Result<(), HandleError> {
  let data = ctx.data.read().await;
  let config = data.get::<ConfigData>().unwrap();

  if let Some(deployment_id) = command
    .data
    .options
    .first()
    .and_then(|option| option.value.as_ref().and_then(Value::as_str))
  {
    let resp = config.cloudflare.get_logs(deployment_id).await?;

    if resp.success {
      let data = resp.result.to_string();
      command
        .create_interaction_response(&ctx.http, |response| {
          response
            .kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|message| {
              message.add_file(AttachmentType::Bytes {
                data: Cow::Borrowed(data.as_bytes()),
                filename: format!("{deployment_id}.log"),
              })
            })
        })
        .await?;
    } else {
      respond_text(
        ctx,
        command,
        &format!("Failed to get logs:\n{}", resp.errors),
      )
      .await?;
    };

    Ok(())
  } else {
    Err(HandleError::OptionNF("deployment_id".to_string()))
  }
}
