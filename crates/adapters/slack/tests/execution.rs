mod common;

use std::net::SocketAddr;

use nautilus_common::clients::ExecutionClient;
use nautilus_model::identifiers::{AccountId, ClientId, Venue};
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
    let (addr, _state) = start_test_server().await.unwrap();
    let mut exec_client = create_test_execution_client(addr);
    assert!(!exec_client.is_connected());
    exec_client.connect().await.unwrap();
    assert!(exec_client.is_connected());
    exec_client.disconnect().await.unwrap();
    assert!(!exec_client.is_connected());
}
