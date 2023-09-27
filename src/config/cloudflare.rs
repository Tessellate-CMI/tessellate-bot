use std::fmt;

use reqwest::Client;
use serde::Deserialize;
use thiserror::Error;
use url::Url;

#[derive(Deserialize)]
pub struct Cloudflare {
  api_key: Box<str>,
  account_id: Box<str>,
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
pub struct CfErrors(Vec<CfError>);

#[derive(Deserialize)]
struct CfError {
  code: u64,
  message: Box<str>,
}

impl fmt::Display for CfErrors {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    for err in &self.0 {
      writeln!(f, "Code {}: {}", err.code, err.message)?;
    }

    Ok(())
  }
}

#[derive(Deserialize)]
pub struct CreateDeployment {
  pub success: bool,
  pub errors: CfErrors,
  pub result: DeploymentResult,
}

#[derive(Deserialize)]
pub struct DeploymentResult {
  pub id: Box<str>,
  pub url: Url,
}

impl Cloudflare {
  pub async fn create_deployment(
    &self,
  ) -> Result<CreateDeployment, CloudflareError> {
    Ok(serde_json::from_slice(
      &Client::new()
        .post(format!(
          "https://api.cloudflare.com/client/v4/accounts/{}/pages/projects/{}/deployments",
          self.account_id,
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

#[derive(Deserialize)]
pub struct Logs {
  pub success: bool,
  pub errors: CfErrors,
  pub result: LogsResult,
}

#[derive(Deserialize)]
pub struct LogsResult {
  data: Vec<LogsResultData>,
}

#[derive(Deserialize)]
struct LogsResultData {
  line: Box<str>,
  ts: Box<str>,
}

impl fmt::Display for LogsResult {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    for log in &self.data {
      writeln!(f, "{}  {}", log.ts, log.line)?;
    }

    Ok(())
  }
}

impl Cloudflare {
  pub async fn get_logs(
    &self,
    deployment_id: &str,
  ) -> Result<Logs, CloudflareError> {
    Ok(serde_json::from_slice(
      &Client::new()
        .get(format!(
          "https://api.cloudflare.com/client/v4/accounts/{}/pages/projects/{}/deployments/{}/history/logs",
          self.account_id,
          self.project_name,
          deployment_id
        ))
        .bearer_auth(self.api_key.to_string())
        .send()
        .await?
        .bytes()
        .await?,
    )?)
  }
}
