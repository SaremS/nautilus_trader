use async_trait::async_trait;
use std::fmt::Debug;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use url::Url;
use nautilus_core::UnixNanos;
use nautilus_common::{
    clients::ExecutionClient,
    messages::execution::{
        BatchCancelOrders, CancelAllOrders, CancelOrder, GenerateFillReports,
        GenerateOrderStatusReport, GenerateOrderStatusReports, GeneratePositionStatusReports,
        ModifyOrder, QueryAccount, QueryOrder, SubmitOrder, SubmitOrderList,
    },
};
use nautilus_model::{
    accounts::AccountAny,
    enums::{LiquiditySide, OmsType},
    events::{
        OrderAccepted, OrderCancelRejected, OrderCanceled, OrderEventAny, OrderExpired,
        OrderFilled, OrderRejected, OrderUpdated,
    },
    identifiers::{
        AccountId, ClientId, ClientOrderId, InstrumentId, StrategyId, TradeId, Venue, VenueOrderId,
    },
    instruments::{Instrument, InstrumentAny},
    orders::Order,
    reports::{ExecutionMassStatus, FillReport, OrderStatusReport, PositionStatusReport},
    types::{AccountBalance, MarginBalance, Money, Price, Quantity},
};

use crate::common::Credential;

#[derive(Clone)]
pub struct SlackClient {
    base_url: Url, 
    channel_id: String,
    api_key: Credential,

    client_id: ClientId,
    account_id: AccountId,
    venue: Venue,
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

        let client_id = ClientId::new("slack_client");
        let account_id = AccountId::new("slack-account");
        let venue = Venue::new("slack");

        Ok(Self {
            base_url,
            channel_id,
            api_key: Credential::new(api_key),
            client_id,
            account_id,
            venue,
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

}

#[async_trait(?Send)]
impl ExecutionClient for SlackClient {
    fn is_connected(&self) -> bool {
        true
    }

    fn client_id(&self) -> ClientId {
        self.client_id
    }

    fn account_id(&self) -> AccountId {
        self.account_id
    }

    fn venue(&self) -> Venue {
       	self.venue 
    }

    fn oms_type(&self) -> OmsType {
        OmsType::Unspecified
    }

    fn get_account(&self) -> Option<AccountAny> {
        None
    }

    /// Generates and publishes the account state event.
    ///
    /// # Errors
    ///
    /// Returns an error if generating the account state fails.
    fn generate_account_state(
        &self,
        balances: Vec<AccountBalance>,
        margins: Vec<MarginBalance>,
        reported: bool,
        ts_event: UnixNanos,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    /// Starts the execution client.
    ///
    /// # Errors
    ///
    /// Returns an error if the client fails to start.
    fn start(&mut self) -> anyhow::Result<()> {
        Ok(())
    }

    /// Stops the execution client.
    ///
    /// # Errors
    ///
    /// Returns an error if the client fails to stop.
    fn stop(&mut self) -> anyhow::Result<()> {
        Ok(())
    }

    /// Connects the client to the execution venue.
    ///
    /// # Errors
    ///
    /// Returns an error if connection fails.
    async fn connect(&mut self) -> anyhow::Result<()> {
        Ok(())
    }

    /// Disconnects the client from the execution venue.
    ///
    /// # Errors
    ///
    /// Returns an error if disconnection fails.
    async fn disconnect(&mut self) -> anyhow::Result<()> {
        Ok(())
    }

    /// Submits a single order command to the execution venue.
    ///
    /// # Errors
    ///
    /// Returns an error if submission fails.
    fn submit_order(&self, cmd: SubmitOrder) -> anyhow::Result<()> {
        Ok(())
    }

    /// Submits a list of orders to the execution venue.
    ///
    /// # Errors
    ///
    /// Returns an error if submission fails.
    fn submit_order_list(&self, cmd: SubmitOrderList) -> anyhow::Result<()> {
        Ok(())
    }

    /// Modifies an existing order.
    ///
    /// # Errors
    ///
    /// Returns an error if modification fails.
    fn modify_order(&self, cmd: ModifyOrder) -> anyhow::Result<()> {
        Ok(())
    }

    /// Cancels a specific order.
    ///
    /// # Errors
    ///
    /// Returns an error if cancellation fails.
    fn cancel_order(&self, cmd: CancelOrder) -> anyhow::Result<()> {
        Ok(())
    }

    /// Cancels all orders.
    ///
    /// # Errors
    ///
    /// Returns an error if cancellation fails.
    fn cancel_all_orders(&self, cmd: CancelAllOrders) -> anyhow::Result<()> {
        Ok(())
    }

    /// Cancels a batch of orders.
    ///
    /// # Errors
    ///
    /// Returns an error if batch cancellation fails.
    fn batch_cancel_orders(&self, cmd: BatchCancelOrders) -> anyhow::Result<()> {
        Ok(())
    }

    /// Queries the status of an account.
    ///
    /// # Errors
    ///
    /// Returns an error if the query fails.
    fn query_account(&self, cmd: QueryAccount) -> anyhow::Result<()> {
        Ok(())
    }

    /// Queries the status of an order.
    ///
    /// # Errors
    ///
    /// Returns an error if the query fails.
    fn query_order(&self, cmd: QueryOrder) -> anyhow::Result<()> {
        Ok(())
    }

    /// Generates a single order status report.
    ///
    /// # Errors
    ///
    /// Returns an error if report generation fails.
    async fn generate_order_status_report(
        &self,
        cmd: &GenerateOrderStatusReport,
    ) -> anyhow::Result<Option<OrderStatusReport>> {
        Ok(None)
    }

    /// Generates multiple order status reports.
    ///
    /// # Errors
    ///
    /// Returns an error if report generation fails.
    async fn generate_order_status_reports(
        &self,
        cmd: &GenerateOrderStatusReports,
    ) -> anyhow::Result<Vec<OrderStatusReport>> {
        Ok(Vec::new())
    }

    /// Generates fill reports based on execution results.
    ///
    /// # Errors
    ///
    /// Returns an error if fill report generation fails.
    async fn generate_fill_reports(
        &self,
        cmd: GenerateFillReports,
    ) -> anyhow::Result<Vec<FillReport>> {
        Ok(Vec::new())
    }

    /// Generates position status reports.
    ///
    /// # Errors
    ///
    /// Returns an error if generation fails.
    async fn generate_position_status_reports(
        &self,
        cmd: &GeneratePositionStatusReports,
    ) -> anyhow::Result<Vec<PositionStatusReport>> {
        Ok(Vec::new())
    }

    /// Generates mass status for executions.
    ///
    /// # Errors
    ///
    /// Returns an error if status generation fails.
    async fn generate_mass_status(
        &self,
        lookback_mins: Option<u64>,
    ) -> anyhow::Result<Option<ExecutionMassStatus>> {
        Ok(None)
    }

    /// Registers an external order for tracking by the execution client.
    ///
    /// This is called after reconciliation creates an external order, allowing the
    /// execution client to track it for subsequent events (e.g., cancellations).
    fn register_external_order(
        &self,
        _client_order_id: ClientOrderId,
        _venue_order_id: VenueOrderId,
        _instrument_id: InstrumentId,
        _strategy_id: StrategyId,
        _ts_init: UnixNanos,
    ) {
        // Default no-op implementation
    }

    /// Handles an instrument update received via the message bus.
    ///
    /// Exec clients that need live instrument updates (e.g. for internal maps)
    /// can override this to process instruments for their venue.
    fn on_instrument(&mut self, _instrument: InstrumentAny) {
        // Default no-op
    }

    /// Calculates the commission for a reconciliation fill.
    ///
    /// Override this method to provide venue-specific commission logic
    /// for inferred fills generated during reconciliation.
    ///
    /// Returns `None` by default, signaling callers to use their own
    /// generic commission formula.
    #[expect(unused_variables)]
    fn calculate_commission(
        &self,
        instrument: &InstrumentAny,
        last_qty: Quantity,
        last_px: Price,
        liquidity_side: LiquiditySide,
    ) -> Option<Money> {
        None
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


