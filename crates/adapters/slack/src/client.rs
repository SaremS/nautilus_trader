use std::fmt::Debug;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use url::Url;
use nautilus_core::UnixNanos;

use crate::common::Credential;

#[derive(Clone)]
pub struct SlackClient {
    base_url: Url, 
    channel_id: String,
    api_key: Credential,
}

#[derive(Serialize)]
struct SlackMessagePayload {
    token: String,
    channel_id: String,
    text: String,
}

#[derive(Deserialize, Debug)]
struct SlackApiResponse {
    ok: bool,
    error: Option<String>,
}

impl Debug for SlackClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!(SlackChannel))
            .field("base_url", &self.base_url)
            .field("channel_id", &self.channel_id)
            .field("api_key", &self.api_key.api_key_masked())
            .finish()
    }
}

impl SlackClient {
    #[must_use]
    pub fn new(channel_id: impl Into<String>, api_key: impl Into<String>) -> Self {
        Self::new_with_base_url("https://slack.com", channel_id, api_key).unwrap()
    }

    #[must_use]
    pub fn new_with_base_url(base_url: impl Into<String>, channel_id: impl Into<String>, api_key: impl Into<String>) -> anyhow::Result<Self> {
        let base_url = base_url.into();
        let base_url = Url::parse(&base_url).map_err(|e| anyhow::anyhow!("Invalid base URL '{}': {}", base_url, e))?;
        let channel_id = channel_id.into();

        Ok(Self {
            base_url,
            channel_id,
            api_key: Credential::new(api_key),
        })
    }

    pub fn set_channel_id(&mut self, channel_id: impl Into<String>) {
        self.channel_id = channel_id.into();
    }

    #[must_use]
    fn build_post_api_url(&self) -> String {
        let mut url = self.base_url.clone();
        url.set_path("/api/chat.postMessage");
        url.to_string()
    }

    #[must_use]
    fn build_post_api_test_url(&self) -> String {
        let mut url = self.base_url.clone();
        url.set_path("/api/api.test");
        url.to_string()
    }

    #[must_use]
    pub async fn send_message(&self, markdown_message: impl Into<String>) -> anyhow::Result<()> {
        let markdown_message = markdown_message.into();

        let payload = SlackMessagePayload {
            token: self.api_key.api_key().to_string(),
            channel_id: self.channel_id.clone(),
            text: markdown_message,
        };

        let api_url = self.build_post_api_url();

        let client = Client::new();
        let response = client
            .post(&api_url)
            .json(&payload)
            .send()
            .await?;
        
        let api_response: SlackApiResponse = response.json().await?;

        if !api_response.ok {
            let error_message = api_response.error.unwrap_or_else(|| "Unknown error".to_string());
            anyhow::bail!("Failed to send message: {}", error_message);
        }        

        Ok(())
    }

    #[must_use]
    pub async fn test_api(&self) -> anyhow::Result<()> {
        let api_url = self.build_post_api_test_url();
        let client = Client::new();
        let response = client
            .post(&api_url)
            .send()
            .await?;

        let api_response: SlackApiResponse = response.json().await?;

        if !api_response.ok {
            let error_message = api_response.error.unwrap_or_else(|| "Unknown error".to_string());
            anyhow::bail!("API test failed: {}", error_message);
        }

        Ok(())
    }

}




#[cfg(test)]
mod tests {
    use rstest::*;

    use super::*;

    #[rstest]
    fn test_slack_client_new_valid() {
        let client = SlackClient::new("channel_id", "api_key");
        assert_eq!(client.base_url, Url::parse("https://slack.com").unwrap());
        assert_eq!(client.channel_id, "channel_id");
        assert_eq!(client.api_key.api_key(), "api_key");
    }
    
    #[rstest]
    fn test_slack_client_new_valid_base_url() {
        let client = SlackClient::new_with_base_url("http://workspace.test", "channel_id", "api_key").unwrap();
        assert_eq!(client.base_url, Url::parse("http://workspace.test").unwrap());
        assert_eq!(client.channel_id, "channel_id");
        assert_eq!(client.api_key.api_key(), "api_key");
    }

    #[rstest]
    fn test_slack_client_new_with_invalid_base_url() {
        let client = SlackClient::new_with_base_url("invalid_url", "channel_id", "api_key");
        assert!(client.is_err());
    }

    #[rstest]
    fn test_slack_client_set_channel() {
        let mut client = SlackClient::new("channel_id", "api_key");
        client.set_channel_id("new_channel_id");
        assert_eq!(client.channel_id, "new_channel_id");
    }
}


