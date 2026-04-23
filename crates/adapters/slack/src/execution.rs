use std::{
    fmt::Debug,
    sync::Mutex,
};
use async_trait::async_trait;
use nautilus_core::{UnixNanos, MUTEX_POISONED};
use nautilus_common::{
    clients::ExecutionClient,
    messages::execution::{
        BatchCancelOrders, CancelAllOrders, CancelOrder, GenerateFillReports,
        GenerateOrderStatusReport, GenerateOrderStatusReports, GeneratePositionStatusReports,
        ModifyOrder, QueryAccount, QueryOrder, SubmitOrder, SubmitOrderList,
    },
    live::get_runtime,
};
use nautilus_model::{
    accounts::AccountAny,
    enums::{LiquiditySide, OmsType},
    identifiers::{
        AccountId, ClientId, ClientOrderId, InstrumentId, StrategyId, Venue, VenueOrderId,
    },
    instruments::InstrumentAny,
    reports::{ExecutionMassStatus, FillReport, OrderStatusReport, PositionStatusReport},
    types::{AccountBalance, MarginBalance, Money, Price, Quantity},
};
use tokio::task::JoinHandle;
use serde::Serialize;

use crate::client::SlackClient;

#[derive(Debug)]
pub struct SlackExecutionClient {
    slack_client: SlackClient,

    client_id: ClientId,
    account_id: AccountId,
    venue: Venue,

    pending_tasks: Mutex<Vec<JoinHandle<()>>>,
}


impl SlackExecutionClient {
    #[must_use]
    pub fn new(channel_id: impl Into<String>, api_key: impl Into<String>) -> Self {
        let slack_client = SlackClient::new(channel_id, api_key);

        let client_id = ClientId::new("slack_client");
        let account_id = AccountId::new("slack-account");
        let venue = Venue::new("slack");

        let pending_tasks = Mutex::new(Vec::new());
        
        Self {
            slack_client,
            client_id,
            account_id,
            venue,
            pending_tasks,
        }
    }

    pub fn new_from_slack_client(slack_client: SlackClient) -> Self {
        let client_id = ClientId::new("slack_client");
        let account_id = AccountId::new("slack-account");
        let venue = Venue::new("slack");

        let pending_tasks = Mutex::new(Vec::new());

        Self {
            slack_client,
            client_id,
            account_id,
            venue,
            pending_tasks,
        }
    }

    fn spawn_task<F>(&self, description: &'static str, fut: F)
    where
        F: Future<Output = anyhow::Result<()>> + Send + 'static,
    {
        let runtime = get_runtime();
        let handle = runtime.spawn(async move {
            if let Err(e) = fut.await {
                log::warn!("{description} failed: {e}");
            }
        });

        let mut tasks = self.pending_tasks.lock().expect(MUTEX_POISONED);
        tasks.retain(|handle| !handle.is_finished());
        tasks.push(handle);
    }

    fn send_slack_message_from_serializable(&self, serializable: impl Serialize, task_description: &'static str) -> anyhow::Result<()> {
        let slack_client = self.slack_client.clone();
        let message_json = serde_json::to_string(&serializable).unwrap_or_else(|_| "Failed to serialize message".to_string());

        let full_message = format!("*{task_description}*:\n```json\n{message_json}\n```");

        self.spawn_task(task_description, async move {
            slack_client.send_message(full_message).await
        });

        Ok(())
    }
}


#[async_trait(?Send)]
impl ExecutionClient for SlackExecutionClient {
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

    fn generate_account_state(
        &self,
        _balances: Vec<AccountBalance>,
        _margins: Vec<MarginBalance>,
        _reported: bool,
        _ts_event: UnixNanos,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn start(&mut self) -> anyhow::Result<()> {
        Ok(())
    }

    fn stop(&mut self) -> anyhow::Result<()> {
        Ok(())
    }

    async fn connect(&mut self) -> anyhow::Result<()> {
        return self.slack_client.test_api().await;
    }

    async fn disconnect(&mut self) -> anyhow::Result<()> {
        Ok(())
    }

    fn submit_order(&self, _cmd: SubmitOrder) -> anyhow::Result<()> {
        Ok(())
    }

    fn submit_order_list(&self, _cmd: SubmitOrderList) -> anyhow::Result<()> {
        self.send_slack_message_from_serializable(_cmd, "SubmitOrderList")
    }

    fn modify_order(&self, _cmd: ModifyOrder) -> anyhow::Result<()> {
        self.send_slack_message_from_serializable(_cmd, "ModifyOrder") 
    }

    fn cancel_order(&self, _cmd: CancelOrder) -> anyhow::Result<()> {
        self.send_slack_message_from_serializable(_cmd, "CancelOrder") 
    }

    fn cancel_all_orders(&self, _cmd: CancelAllOrders) -> anyhow::Result<()> {
        self.send_slack_message_from_serializable(_cmd, "CancelAllOrders") 
    }

    fn batch_cancel_orders(&self, _cmd: BatchCancelOrders) -> anyhow::Result<()> {
        self.send_slack_message_from_serializable(_cmd, "BatchCancelOrders") 
    }

    fn query_account(&self, _cmd: QueryAccount) -> anyhow::Result<()> {
        self.send_slack_message_from_serializable(_cmd, "QueryAccount") 
    }

    fn query_order(&self, _cmd: QueryOrder) -> anyhow::Result<()> {
        self.send_slack_message_from_serializable(_cmd, "QueryOrder") 
    }

    /// Generates a single order status report.
    ///
    /// # Errors
    ///
    /// Returns an error if report generation fails.
    async fn generate_order_status_report(
        &self,
        _cmd: &GenerateOrderStatusReport,
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
        _cmd: &GenerateOrderStatusReports,
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
        _cmd: GenerateFillReports,
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
        _cmd: &GeneratePositionStatusReports,
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
        _lookback_mins: Option<u64>,
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
