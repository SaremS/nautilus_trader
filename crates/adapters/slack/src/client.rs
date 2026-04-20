use std::fmt::Debug;
use ahash::AHashMap;

use crate::common::SlackChannel;


#[derive(Clone)]
pub struct SlackClient {
   slack_channels: AHashMap<String, SlackChannel> 
}

impl Debug for SlackClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!(SlackClient))
            .field("slack_channels", &self.slack_channels)
            .finish()
    }
}

impl SlackClient {
    #[must_use]
    pub fn new(channels: Vec<SlackChannel>) -> Self {
        let mut map = AHashMap::new(); 
        for channel in channels {
            map.insert(channel.get_channel_id(), channel);
        }
        Self {
            slack_channels: map
        }
    }

    pub fn add_channel(&mut self, channel: SlackChannel) {
        self.slack_channels.insert(channel.get_channel_id(), channel);
    }
    
    #[must_use]
    pub fn get_channel_names(&self) -> Vec<String> {
        self.slack_channels.keys().cloned().collect()
    }

    #[must_use]
    pub fn get_channel(&self, channel_id: &str) -> Option<&SlackChannel> {
        self.slack_channels.get(channel_id)
    }
}


#[cfg(test)]
mod tests {
    use rstest::*;

    use super::*;
    
    #[rstest]
    fn test_slack_client_new() {
        let channel1 = SlackChannel::new("channel1", "api_key1");
        let channel2 = SlackChannel::new("channel2", "api_key2");
        let mut client = SlackClient::new(vec![channel1.clone(), channel2.clone()]);

        assert_eq!(client.slack_channels.len(), 2);
        assert_eq!(client.slack_channels.get("channel1").unwrap().api_key(), "api_key1");
        assert_eq!(client.slack_channels.get("channel2").unwrap().api_key(), "api_key2");
    }

    #[rstest]
    fn test_slack_client_add_channel() {
        let channel1 = SlackChannel::new("channel1", "api_key1");
        let mut client = SlackClient::new(vec![channel1.clone()]);

        let channel2 = SlackChannel::new("channel2", "api_key2");
        client.add_channel(channel2.clone());

        assert_eq!(client.slack_channels.len(), 2);
        assert_eq!(client.slack_channels.get("channel1").unwrap().api_key(), "api_key1");
        assert_eq!(client.slack_channels.get("channel2").unwrap().api_key(), "api_key2");
    }

    #[rstest]
    fn test_slack_client_get_channel_names() {
        let channel1 = SlackChannel::new("channel1", "api_key1");
        let channel2 = SlackChannel::new("channel2", "api_key2");

        let client = SlackClient::new(vec![channel1.clone(), channel2.clone()]);
        let channel_names = client.get_channel_names();

        assert_eq!(channel_names.len(), 2);
        assert!(channel_names.contains(&"channel1".to_string()));
        assert!(channel_names.contains(&"channel2".to_string()));
    }

    #[rstest]
    fn test_slack_client_get_channel() {
        let channel1 = SlackChannel::new("channel1", "api_key1");
        let channel2 = SlackChannel::new("channel2", "api_key2");

        let client = SlackClient::new(vec![channel1.clone(), channel2.clone()]);
        let retrieved_channel1 = client.get_channel("channel1").unwrap();
        let retrieved_channel2 = client.get_channel("channel2").unwrap();

        assert_eq!(retrieved_channel1.api_key(), "api_key1");
        assert_eq!(retrieved_channel2.api_key(), "api_key2");
    }
}


