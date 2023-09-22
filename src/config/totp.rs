use std::time::SystemTimeError;

use serde::{de, Deserialize, Deserializer};
use tabled::Tabled;
use thiserror::Error;
use totp_rs::{Algorithm, Secret, TotpUrlError, TOTP};

#[derive(Deserialize, Tabled)]
pub struct Totp {
  #[tabled(rename = "Name")]
  pub name: Box<str>,
  #[tabled(rename = "Description")]
  desc: Box<str>,
  #[serde(deserialize_with = "deserialize_secret")]
  #[tabled(skip)]
  secret: Vec<u8>,
}

fn deserialize_secret<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
where
  D: Deserializer<'de>,
{
  let s: String = Deserialize::deserialize(deserializer)?;
  Secret::Encoded(s.replace(' ', ""))
    .to_bytes()
    .map_err(de::Error::custom)
}

#[derive(Debug, Error)]
pub enum TotpError {
  #[error(transparent)]
  TotpUrl(#[from] TotpUrlError),

  #[error(transparent)]
  SystemTime(#[from] SystemTimeError),
}

impl Totp {
  pub fn generate_totp(&self) -> Result<String, TotpError> {
    Ok(
      TOTP::new(Algorithm::SHA1, 6, 1, 30, self.secret.clone())?
        .generate_current()?,
    )
  }
}
