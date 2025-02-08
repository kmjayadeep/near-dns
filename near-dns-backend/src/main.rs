use near_api::{AccountId, Contract, Data, NetworkConfig};
use serde::{Deserialize, Serialize};

mod adguard;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DNSRecord {
    owner: AccountId,
    a: String,
    aaaa: String,
}

async fn get_near_state() -> Result<Vec<(String, DNSRecord)>, Box<dyn std::error::Error>> {
    let network = NetworkConfig::testnet();

    let contract_id: AccountId = "near-dns-staging.testnet".parse().unwrap();

    // Load the contract
    let contract = Contract(contract_id.clone());

    let result: Data<Vec<(String, DNSRecord)>> = contract
        .call_function("get_all_domains", ())
        .unwrap()
        .read_only()
        .fetch_from(&network)
        .await
        .unwrap();

    Ok(result.data)
}

async fn run() {
    let domains = get_near_state().await.unwrap();
    println!("Domains In NEAR:");
    for (name, record) in domains.clone() {
        println!("- Name: {}, A: {}, AAAA: {}", name, record.a, record.aaaa);
    }
    adguard::reconcile(domains).await;
}

#[tokio::main]
async fn main() {
    println!("Starting Near-DNS Backend");
    loop {
        run().await;
        tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
    }
}
