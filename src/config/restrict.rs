use std::collections::HashMap;

use serde::Deserialize;
use serenity::{
  model::prelude::{
    application_command::ApplicationCommandInteraction, ChannelId, RoleId,
    UserId,
  },
  prelude::Context,
};

#[derive(Deserialize)]
pub struct Restrict(HashMap<Box<str>, Allow>);

#[derive(Deserialize)]
struct Allow {
  channels: Option<Vec<ChannelId>>,
  roles: Option<Vec<RoleId>>,
  users: Option<Vec<UserId>>,
}

impl Restrict {
  pub async fn allowed(
    &self,
    ctx: &Context,
    command: &ApplicationCommandInteraction,
  ) -> Result<bool, serenity::Error> {
    let Some(allow) = self.0.get(command.data.name.as_str()) else {
      return Ok(true);
    };

    if let Some(users) = &allow.users {
      if users.contains(&command.user.id) {
        return Ok(true);
      }
    }

    if let Some(channels) = &allow.channels {
      if channels.contains(&command.channel_id) {
        return Ok(true);
      }
    }

    if let Some(roles) = &allow.roles {
      for role in roles {
        if command
          .user
          .has_role(ctx, command.guild_id.unwrap(), role)
          .await?
        {
          return Ok(true);
        }
      }
    }

    Ok(false)
  }
}
