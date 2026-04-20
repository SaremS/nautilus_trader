use std::fmt::Debug;

use nautilus_core::string::REDACTED;
use zeroize::ZeroizeOnDrop;


#[derive(Clone, ZeroizeOnDrop)]
pub struct SlackChannel {
    channel_id: String,
    api_key: Box<[u8]>,
}

impl Debug for SlackChannel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!(SlackChannel))
            .field("channel_id", &self.channel_id)
            .field("api_key", &REDACTED)
            .finish()
    }
}

impl SlackChannel {
    /// Creates a new [`Credential`] instance from the API key.
    #[must_use]
    pub fn new(channel_id: impl Into<String>, api_key: impl Into<String>) -> Self {
        let channel_id = channel_id.into();
        let api_key_bytes = api_key.into().into_bytes();

        Self {
            channel_id,
            api_key: api_key_bytes.into_boxed_slice(),
        }
    }

    /// Returns the API key associated with this credential.
    ///
    /// # Panics
    ///
    /// This method should never panic as the API key is always valid UTF-8,
    /// having been created from a String.
    #[must_use]
    pub fn api_key(&self) -> &str {
        std::str::from_utf8(&self.api_key).expect("API key is valid UTF-8")
    }

    /// Returns a masked version of the API key for logging purposes.
    ///
    /// Shows first 4 and last 4 characters with ellipsis in between.
    /// For keys shorter than 8 characters, shows asterisks only.
    #[must_use]
    pub fn api_key_masked(&self) -> String {
        nautilus_core::string::mask_api_key(self.api_key())
    }
    
    /// Returns the channel_id
    #[must_use]
    pub fn channel_id(&self) -> String {
        self.channel_id.clone()
    }
}



#[cfg(test)]
mod tests {
    use rstest::*;

    use super::*;

    #[rstest]
    fn test_slack_channel_api_key_masked_short() {
        let credential = SlackChannel::new("test_channel", "short");
        assert_eq!(credential.api_key_masked(), "*****");
    }

    #[rstest]
    fn test_slack_channel_api_key_masked_long() {
        let credential = SlackChannel::new("test_channel", "abcdefghijklmnop");
        assert_eq!(credential.api_key_masked(), "abcd...mnop");
    }

    #[rstest]
    fn test_slack_channel_debug_redaction() {
        let credential = SlackChannel::new("test_channel", "test_api_key");
        let debug_str = format!("{credential:?}");
        assert!(debug_str.contains(REDACTED));
        assert!(!debug_str.contains("test_api_key"));
    }

    #[rstest]
    fn test_slack_channel_channel_id() {
        let credential = SlackChannel::new("test_channel", "test_api_key");
        let debug_str = format!("{credential:?}");
        assert!(debug_str.contains("test_channel"));
        assert!(debug_str.contains("channel_id"));
    }
}
