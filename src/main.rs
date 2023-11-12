mod commands;
mod config;
mod handler;

use std::{env, sync::Arc};

use serenity::{
  prelude::{GatewayIntents, TypeMapKey},
  Client,
};

use config::Config;
use handler::Handler;

struct ConfigData;

impl TypeMapKey for ConfigData {
  type Value = Arc<Config>;
}

#[tokio::main]
async fn main() {
  tracing_subscriber::fmt().init();

  let config = match Config::from_file(
    env::var("CONFIG_PATH").unwrap_or("config.yml".to_string()),
  ) {
    Ok(c) => c,
    Err(err) => {
      eprintln!("Config error: {err}");
      return;
    }
  };

  let mut client =
    match Client::builder(config.token.clone(), GatewayIntents::empty())
      .event_handler(Handler)
      .await
    {
      Ok(c) => c,
      Err(err) => {
        eprintln!("Client creation error: {err}");
        return;
      }
    };

  client
    .data
    .write()
    .await
    .insert::<ConfigData>(Arc::from(config));

  if let Err(err) = client.start().await {
    eprintln!("Client error: {err}");
  }
}
