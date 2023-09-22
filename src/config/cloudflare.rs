use reqwest::Client;
use serde::Deserialize;
use thiserror::Error;
use url::Url;

#[derive(Deserialize)]
pub struct Cloudflare {
  api_key: Box<str>,
  account_identifier: Box<str>,
  project_name: Box<str>,
}

#[derive(Debug, Error)]
pub enum CloudflareError {
  #[error(transparent)]
  Reqwest(#[from] reqwest::Error),

  #[error(transparent)]
  Serde(#[from] serde_json::Error),
}

#[derive(Deserialize)]
pub struct CreateDeployment {
  pub success: bool,
  pub result: DeploymentResult,
  pub errors: Vec<DeploymentError>,
}

#[derive(Deserialize)]
pub struct DeploymentResult {
  pub id: Box<str>,
  pub url: Url,
}

#[derive(Debug, Deserialize)]
pub struct DeploymentError {
  pub code: u64,
  pub message: Box<str>,
}

impl Cloudflare {
  pub async fn create_deployment(
    &self,
  ) -> Result<CreateDeployment, CloudflareError> {
    Ok(serde_json::from_slice(
      &Client::new()
        .post(format!(
          "https://api.cloudflare.com/client/v4/accounts/{}/pages/projects/{}/deployments",
          self.account_identifier,
          self.project_name
        ))
        .bearer_auth(self.api_key.to_string())
        .send()
        .await?
        .bytes()
        .await?,
    )?)
  }
}
