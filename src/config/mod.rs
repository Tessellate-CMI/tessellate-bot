pub mod cloudflare;
pub mod restrict;
pub mod totp;

use std::{fs::File, io, path::Path};

use serde::Deserialize;
use serenity::model::prelude::GuildId;
use thiserror::Error;

use cloudflare::Cloudflare;
use restrict::Restrict;
use totp::Totp;

#[derive(Deserialize)]
pub struct Config {
  pub token: Box<str>,
  pub guild: GuildId,
  pub cloudflare: Cloudflare,
  pub totp: Vec<Totp>,
  pub restrict: Restrict,
}

#[derive(Debug, Error)]
pub enum ConfigError {
  #[error(transparent)]
  IO(#[from] io::Error),

  #[error(transparent)]
  Deserialize(#[from] serde_yaml::Error),
}

impl Config {
  pub fn from_file(path: impl AsRef<Path>) -> Result<Config, ConfigError> {
    Ok(serde_yaml::from_reader::<_, Config>(File::open(path)?)?)
  }
}
