use serenity::{
  async_trait,
  model::prelude::{Interaction, Ready},
  prelude::{Context, EventHandler},
};
use tracing::{error, info};

use crate::{commands, ConfigData};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
  async fn ready(&self, ctx: Context, ready: Ready) {
    info!("Connected: {}", ready.user.name);

    let data = ctx.data.read().await;
    let config = data.get::<ConfigData>().unwrap();

    match config
      .guild
      .set_application_commands(&ctx.http, commands::register)
      .await
    {
      Ok(cmds) => {
        info!(
          "Registered guild commands: {:?}",
          cmds.iter().map(|c| &c.name).collect::<Vec<_>>()
        );
      }
      Err(err) => error!("Guild commands registration error: {err}"),
    }
  }

  async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
    if let Interaction::ApplicationCommand(command) = interaction {
      info!("Received command interaction: {command:?}");

      if let Err(err) = commands::handle(&ctx, &command).await {
        error!("Slash command error: {err}");
      }
    }
  }
}
