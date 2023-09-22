use serenity::{
  builder::CreateApplicationCommand,
  model::prelude::application_command::ApplicationCommandInteraction,
  prelude::Context,
};

use super::{respond_text, HandleError};

pub const NAME: &str = "ping";
const DESCRIPTION: &str = "A ping command";

pub fn register(
  command: &mut CreateApplicationCommand,
) -> &mut CreateApplicationCommand {
  command.name(NAME).description(DESCRIPTION)
}

pub async fn run(
  ctx: &Context,
  command: &ApplicationCommandInteraction,
) -> Result<(), HandleError> {
  Ok(respond_text(ctx, command, "Pong!").await?)
}
