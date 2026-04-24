mod common;

use std::{
    net::SocketAddr,
    sync::atomic::Ordering
};

use nautilus_core::{UUID4, UnixNanos};

use nautilus_common::{
    clients::ExecutionClient,
    messages::execution::SubmitOrder,
};
use nautilus_model::{
    identifiers::{AccountId, ClientId, Venue, TraderId, ClientOrderId, StrategyId, InstrumentId},
    orders::{LimitOrder, OrderAny},
    enums::{OrderSide, TimeInForce},
    types::{Price, Quantity}
};
use nautilus_slack::{client::SlackClient, execution::SlackExecutionClient};

use rstest::rstest;

use crate::common::server::start_test_server;

fn create_test_execution_client(addr: SocketAddr) -> SlackExecutionClient {
    let base_url = format!("http://{addr}");
    let slack_client = SlackClient::new_with_base_url(base_url, "channel_id", "api_key").unwrap();
    let slack_execution_client = SlackExecutionClient::new_from_slack_client(slack_client);

    return slack_execution_client;
}

#[rstest]
#[tokio::test]
async fn test_exec_client_create() {
    let (addr, _state) = start_test_server().await.unwrap();
    let exec_client = create_test_execution_client(addr);
    assert_eq!(exec_client.client_id(), ClientId::new("slack_client"));
    assert_eq!(exec_client.account_id(), AccountId::new("slack-account"));
    assert_eq!(exec_client.venue(), Venue::new("slack"));
}

#[rstest]
#[tokio::test]
async fn test_exec_client_connect_disconnect() {
    let (addr, state) = start_test_server().await.unwrap();
    let mut exec_client = create_test_execution_client(addr);
    assert!(!exec_client.is_connected());
    assert!(state.connection_count.load(Ordering::Relaxed) == 0);
    exec_client.connect().await.unwrap();
    assert!(exec_client.is_connected());
    assert!(state.connection_count.load(Ordering::Relaxed) == 1);
    exec_client.disconnect().await.unwrap();
    assert!(!exec_client.is_connected());
}

#[rstest]
#[tokio::test]
async fn test_exec_client_send_message() {
    let (addr, state) = start_test_server().await.unwrap();
    let mut exec_client = create_test_execution_client(addr);

    let trader_id = TraderId::from("TESTER-001");
    let instrument_id = InstrumentId::from("BTC-USD.Test");
    let strategy_id = StrategyId::from("S-001");
    let coid = ClientOrderId::from("COID-001");

    let order = LimitOrder::new(
        trader_id,
        strategy_id,
        instrument_id,
        coid,
        OrderSide::Buy,
        Quantity::from("1"),
        Price::from("50000.00"),
        TimeInForce::Gtc,
        None,
        false,
        false,
        false,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        UUID4::new(),
        UnixNanos::default(),
    );

    let mut order_any: OrderAny = order.into();
    let submit_order = SubmitOrder::from_order(
        &order_any,
        trader_id,
        Some(ClientId::new("test-client-id")),
        None,
        UUID4::new(),
        UnixNanos::default(),
    );

    let result = exec_client.submit_order(submit_order).unwrap();

    tokio::time::sleep(std::time::Duration::from_millis(500)).await;
    let messages = state.messages_received.lock().await;
    assert_eq!(messages.len(), 1);
    let message = &messages[0];
    assert!(message.contains("client_id"));
}
