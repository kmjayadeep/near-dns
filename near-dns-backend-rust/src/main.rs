use near_api::{AccountId, Contract, Data, NetworkConfig};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct DNSRecord {
    owner: AccountId,
    a: String,
    aaaa: String,
}

async fn get_all_domains() -> Result<Vec<(String, DNSRecord)>, Box<dyn std::error::Error>> {
    let network = NetworkConfig::testnet();

    let contract_id: AccountId = "near-dns-test3.testnet".parse().unwrap();

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

#[tokio::main]
async fn main() {
    match get_all_domains().await {
        Ok(domains) => {
            println!("Domains:");
            for (name, record) in domains {
                println!("- Name: {}, A: {}, AAAA: {}", name, record.a, record.aaaa);
            }
        }
        Err(err) => {
            eprintln!("Error fetching domains: {}", err);
        }
    }
}
