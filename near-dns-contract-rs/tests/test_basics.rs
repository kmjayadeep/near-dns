use near_workspaces::{network::Sandbox, Contract, Worker};
use serde_json::json;

async fn init() -> anyhow::Result<(Worker<Sandbox>, Contract)> {
    let worker = near_workspaces::sandbox().await?;
    let contract = worker
        .dev_deploy(include_bytes!("../target/near/near_dns.wasm"))
        .await?;

    Ok((worker, contract))
}


#[tokio::test]
async fn test_domain_registration() -> anyhow::Result<()> {
    let (worker, contract) = init().await?;

    let user_account = worker.dev_create_account().await?;

    let outcome = user_account
        .call(contract.id(), "register_domain")
        .args_json(json!({"domain": "router", "a": "192.168.1.1", "aaaa": "::1"}))
        .transact()
        .await?;
    assert!(outcome.is_success());

    let user_message_outcome = contract
        .view("get_domain")
        .args_json(json!({"domain":"router"}))
        .await?;
    assert_eq!(user_message_outcome.json::<String>()?, "Hello World!");

    Ok(())
}
