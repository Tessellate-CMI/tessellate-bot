mod deploy;
mod logs;
mod ping;
mod totp;

use serenity::{
  builder::CreateApplicationCommands,
  model::prelude::{
    application_command::ApplicationCommandInteraction, InteractionResponseType,
  },
  prelude::Context,
};
use thiserror::Error;
use tracing::warn;

use crate::{
  config::{cloudflare::CloudflareError, totp::TotpError},
  ConfigData,
};

macro_rules! register {
  ($commands:expr, $($cmd:ident),*) => {
    $commands
      $(
        .create_application_command(|c| $cmd::register(c))
      )*
  };
}

pub fn register(
  commands: &mut CreateApplicationCommands,
) -> &mut CreateApplicationCommands {
  register!(commands, ping, deploy, logs, totp)
}

async fn respond_text(
  ctx: &Context,
  command: &ApplicationCommandInteraction,
  text: &str,
) -> serenity::Result<()> {
  command
    .create_interaction_response(&ctx.http, |response| {
      response
        .kind(InteractionResponseType::ChannelMessageWithSource)
        .interaction_response_data(|message| message.content(text))
    })
    .await
}

#[derive(Debug, Error)]
pub enum HandleError {
  #[error(transparent)]
  Serenity(#[from] serenity::Error),

  #[error(transparent)]
  Totp(#[from] TotpError),

  #[error(transparent)]
  Cloudflare(#[from] CloudflareError),

  #[error("Option not found: {0}")]
  OptionNF(String),
}

macro_rules! handle {
  ($ctx:ident, $command:ident, $($cmd:ident),*) => {
      match $command.data.name.as_str() {
          $(
              $cmd::NAME => $cmd::run($ctx, $command).await,
          )*
          _ => Ok(respond_text($ctx, $command, "Not Implemented!").await?),
      }
  };
}

pub async fn handle(
  ctx: &Context,
  command: &ApplicationCommandInteraction,
) -> Result<(), HandleError> {
  let data = ctx.data.read().await;
  let config = data.get::<ConfigData>().unwrap();

  if config.restrict.allowed(ctx, command).await? {
    handle!(ctx, command, ping, deploy, logs, totp)
  } else {
    warn!("Denied access: {command:?}");
    Ok(respond_text(ctx, command, "**Access Denied!**").await?)
  }
}
