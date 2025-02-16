use near_api::{AccountId, Contract, Data, NetworkConfig};
use serde::{Deserialize, Serialize};
use tokio::task;

mod adguard;
mod cloudflaredns;

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

async fn reconcile_adguard() {
    let domains = get_near_state().await.unwrap();
    println!("Domains In NEAR:");
    for (name, record) in domains.clone() {
        println!("- Name: {}, A: {}, AAAA: {}", name, record.a, record.aaaa);
    }
    adguard::reconcile(domains).await;
}

async fn reconcile_cloudflare() {
    let domains = get_near_state().await.unwrap();
    println!("Domains In NEAR:");
    for (name, record) in domains.clone() {
        println!("- Name: {}, A: {}, AAAA: {}", name, record.a, record.aaaa);
    }
    task::spawn_blocking(move || {
        cloudflaredns::reconcile(domains);
    })
    .await
    .unwrap();
}

#[tokio::main]
async fn main() {
    println!("Starting Near-DNS Backend");
    let env_type = std::env::var("ENV_TYPE").expect("Missing ENV_TYPE env variable");
    let reconcile_interval = std::env::var("RECONCILE_INTERVAL")
        .expect("Missing RECONCILE_INTERVAL env variable")
        .parse::<u64>()
        .expect("RECONCILE_INTERVAL must be a valid number");

    loop {
        match env_type.as_str() {
            "staging" => {
                println!("Reconciling Adguard");
                reconcile_adguard().await;
            }
            "production" => {
                println!("Reconciling Cloudflare");
                reconcile_cloudflare().await;
            }
            _ => {
                panic!("Unknown ENV_TYPE: {}", env_type);
            }
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(reconcile_interval)).await;
    }
}
