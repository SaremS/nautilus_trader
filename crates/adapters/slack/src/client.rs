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
}

