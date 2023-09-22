use serde_json::value::Value;
use serenity::{
  builder::CreateApplicationCommand,
  model::prelude::{
    application_command::ApplicationCommandInteraction,
    command::CommandOptionType,
  },
  prelude::Context,
};
use tabled::{settings::Style, Table};

use super::{respond_text, HandleError};
use crate::ConfigData;

pub const NAME: &str = "totp";
const DESCRIPTION: &str = "TOTP command";

pub fn register(
  command: &mut CreateApplicationCommand,
) -> &mut CreateApplicationCommand {
  command
    .name(NAME)
    .description(DESCRIPTION)
    .create_option(|option| {
      option
        .kind(CommandOptionType::String)
        .name("app")
        .description("App Name")
    })
}

pub async fn run(
  ctx: &Context,
  command: &ApplicationCommandInteraction,
) -> Result<(), HandleError> {
  let data = ctx.data.read().await;
  let config = data.get::<ConfigData>().unwrap();

  if let Some(option) = command
    .data
    .options
    .first()
    .and_then(|option| option.value.as_ref().and_then(Value::as_str))
  {
    if let Some(app) = config
      .totp
      .iter()
      .find(|t| t.name.as_bytes() == option.as_bytes())
    {
      Ok(respond_text(ctx, command, &app.generate_totp()?).await?)
    } else {
      Ok(respond_text(ctx, command, "App not found!").await?)
    }
  } else {
    Ok(
      respond_text(
        ctx,
        command,
        &format!(
          "```\n{}\n```",
          Table::new(&config.totp).with(Style::markdown())
        ),
      )
      .await?,
    )
  }
}
