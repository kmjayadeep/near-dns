use near_api::{AccountId, Contract, Data, NetworkConfig};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
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

async fn list_records() -> Result<HashMap<String, String>, reqwest::Error> {
    println!("Listing domains");
    let url = "http://gatekeeper.cosmos.cboxlab.com/control/rewrite/list";
    let adguard_password =
        std::env::var("ADGUARD_PASSWORD").expect("Missing ADGUARD_PASSWORD env var");

    let client = Client::new();
    let response = client
        .get(url)
        .header("Content-Type", "application/json")
        .basic_auth("admin", Option::from(adguard_password))
        .send()
        .await?;

    let body: Vec<HashMap<String, String>> = response.json().await?;
    println!("Got {} domains", body.len());

    let mut records = HashMap::new();
    for r in body {
        let domain = r["domain"].as_str();
        records.insert(domain.to_string(), r["answer"].to_string());
    }

    Ok(records)
}

async fn add_record(domain: String, answer: String) -> Result<(), reqwest::Error> {
    println!("Adding record for {}, answer {}", domain, answer);
    let url = "http://gatekeeper.cosmos.cboxlab.com/control/rewrite/add";
    let adguard_password =
        std::env::var("ADGUARD_PASSWORD").expect("Missing ADGUARD_PASSWORD env var");

    let rewrite_rule = json!({
        "domain": domain,
        "answer": answer,
    });

    let client = Client::new();
    let response = client
        .post(url)
        .header("Content-Type", "application/json")
        .basic_auth("admin", Option::from(adguard_password))
        .json(&rewrite_rule)
        .send()
        .await?;

    if response.status().is_success() {
        println!("Record added successfully");
    } else {
        println!("Failed to add record");
    }

    Ok(())
}

async fn update_record(
    domain: String,
    existing: String,
    answer: String,
) -> Result<(), reqwest::Error> {
    println!("Updating record for {}, answer {}", domain, answer);
    let url = "http://gatekeeper.cosmos.cboxlab.com/control/rewrite/update";
    let adguard_password =
        std::env::var("ADGUARD_PASSWORD").expect("Missing ADGUARD_PASSWORD env var");

    let payload = json!({
        "target": {
            "domain": domain,
            "answer": existing,
        },
        "update": {
            "domain": domain,
            "answer": answer,
        }
    });

    println!("Payload: {}", payload);

    let client = Client::new();
    let _response = client
        .put(url)
        .header("Content-Type", "application/json")
        .basic_auth("admin", Option::from(adguard_password))
        .json(&payload)
        .send()
        .await?;

    Ok(())
}

#[tokio::main]
async fn main() {
    println!("Starting Near-DNS Backend");
    let domains = get_all_domains().await.unwrap();
    println!("Domains:");
    for (name, record) in domains.clone() {
        println!("- Name: {}, A: {}, AAAA: {}", name, record.a, record.aaaa);
    }
    println!("Domains in Adguard:");
    let existing = list_records().await.unwrap();
    for (name, record) in existing.clone() {
        println!("- Name: {}, Record: {}", name, record);
    }

    // Update the DNS records
    for (n, record) in domains {
        let answer = if !record.a.is_empty() {
            record.a.clone()
        } else {
            record.aaaa.clone()
        };
        let name = n + ".local";
        if existing.contains_key(&name) {
            if existing.get(&name).unwrap() == &answer {
                println!("Record for {} is up to date", name);
                continue;
            }
            let result =
                update_record(name.clone(), existing.get(&name).unwrap().clone(), answer).await;
            if result.is_err() {
                println!("Failed to update record for {}", name);
            }
        } else {
            let result = add_record(name.clone(), answer).await;
            if result.is_err() {
                println!("Failed to add record for {}", name);
            }
        }
    }
}
