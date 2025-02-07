use near_dns::DNSRecord;
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

    let result = contract
        .view("get_domain")
        .args_json(json!({"domain":"router"}))
        .await?;

    let domain = result.json::<DNSRecord>()?;

    assert_eq!(domain.owner, user_account.id().to_string());
    assert_eq!(domain.a, "192.168.1.1");
    assert_eq!(domain.aaaa, "::1");

    // Try updating with a different user
    let user_account2 = worker.dev_create_account().await?;
    let outcome2 = user_account2
        .call(contract.id(), "register_domain")
        .args_json(json!({"domain": "router", "a": "192.168.1.1", "aaaa": "::1"}))
        .transact()
        .await?;
    assert!(outcome2.is_failure());

    Ok(())
}

#[tokio::test]
async fn test_domain_deletion() -> anyhow::Result<()> {
    let (worker, contract) = init().await?;

    let user_account = worker.dev_create_account().await?;
    let user_account2 = worker.dev_create_account().await?;

    // Register a domain
    let outcome = user_account
        .call(contract.id(), "register_domain")
        .args_json(json!({"domain": "router", "a": "192.168.1.1", "aaaa": "::1"}))
        .transact()
        .await?;
    assert!(outcome.is_success());

    // Try deleting the domain with a different user
    let result = user_account2
        .call(contract.id(), "delete_domain")
        .args_json(json!({"domain":"router"}))
        .transact()
        .await?;
    assert!(result.is_failure());

    // Delete the domain with the correct user
    let result2 = user_account
        .call(contract.id(), "delete_domain")
        .args_json(json!({"domain": "router"}))
        .transact()
        .await?;
    assert!(result2.is_success());

    Ok(())
}
