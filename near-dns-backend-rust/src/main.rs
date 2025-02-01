use serde::{Deserialize, Serialize};
use near_api::{AccountId, Contract, NetworkConfig, Data};

#[derive(Serialize, Deserialize, Debug)]
struct Domain {
    owner: AccountId,
    A: String,
    AAAA: String,
}

async fn get_all_domains() -> Result<String, Box<dyn std::error::Error>> {
    let network = NetworkConfig::testnet();

    let contract_id: AccountId = "near-dns-test2.testnet".parse().unwrap();

    // Load the contract
    let contract = Contract(contract_id.clone());

    let result : Data<String> = contract
    .call_function("get_all_domains", ())
    .unwrap()
    .read_only()
    .fetch_from(&network)
    .await
    .unwrap();

    let raw_result = serde_json::to_string(&result).unwrap();
    println!("{}", raw_result);

    Ok(String::from("ok"))
}

#[tokio::main]
async fn main() {
    match get_all_domains().await {
        Ok(_domains) => {
            println!("Domains:");
            // for domain in domains {
            //     println!("- Name: {}, Owner: {}", domain.A, domain.owner);
            // }
        }
        Err(err) => {
            eprintln!("Error fetching domains: {}", err);
        }
    }
}