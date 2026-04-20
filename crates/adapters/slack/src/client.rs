use std::fmt::Debug;
use ahash::AHashSet;

use crate::common::Credential;

#[derive(Clone)]
pub struct SlackClient {
    workspace_name: String, 
    channels: AHashSet<String>,
    api_key: Credential,
}

impl Debug for SlackClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!(SlackChannel))
            .field("workspace_name", &self.workspace_name)
            .field("channels", &self.channels)
            .field("api_key", &self.api_key.api_key_masked())
            .finish()
    }
}

impl SlackClient {
    #[must_use]
    pub fn new(workspace_name: impl Into<String>, channels: AHashSet<String>, api_key: impl Into<String>) -> Self {
        let workspace_name = workspace_name.into();

        Self {
            workspace_name,
            channels,
            api_key: Credential::new(api_key),
        }
    }

    pub fn add_channel(&mut self, channel: impl Into<String>) {
        self.channels.insert(channel.into());
    }
    
    #[must_use]
    pub fn get_channel_names(&self) -> Vec<String> {
        self.channels.iter().cloned().collect()
    }

}


#[cfg(test)]
mod tests {
    use rstest::*;

    use super::*;
    
    #[rstest]
    fn test_slack_client_new() {
        let channels = AHashSet::from_iter(vec!["channel1".to_string(), "channel2".to_string()]);
        let client = SlackClient::new("workspace", channels, "api_key");
        assert_eq!(client.workspace_name, "workspace".to_string());
    }

    #[rstest]
    fn test_slack_client_add_channel() {
        let mut client = SlackClient::new("workspace", AHashSet::new(), "api_key");
        client.add_channel("channel1");
        client.add_channel("channel2");

        let channel_names = client.get_channel_names();
        assert_eq!(channel_names.len(), 2);
        assert!(channel_names.contains(&"channel1".to_string()));
        assert!(channel_names.contains(&"channel2".to_string()));
    }

    #[rstest]
    fn test_slack_client_get_channel_names() {
        let channels = AHashSet::from_iter(vec!["channel1".to_string(), "channel2".to_string()]);
        let client = SlackClient::new("workspace", channels, "api_key");

        let channel_names = client.get_channel_names();
        assert_eq!(channel_names.len(), 2);
        assert!(channel_names.contains(&"channel1".to_string()));
        assert!(channel_names.contains(&"channel2".to_string()));
    }

    #[rstest]
    fn test_slack_client_get_channel() {
        let channels = AHashSet::from_iter(vec!["channel1".to_string(), "channel2".to_string()]);
        let client = SlackClient::new("workspace", channels, "api_key");

        let channel_names = client.get_channel_names();
        assert_eq!(channel_names.len(), 2);
        assert!(channel_names.contains(&"channel1".to_string()));
        assert!(channel_names.contains(&"channel2".to_string()));
    }
}


