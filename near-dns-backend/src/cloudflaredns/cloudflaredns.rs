use crate::DNSRecord;

use reqwest::Client;
use serde::Deserialize;
use std::collections::HashMap;

const CLOUDFLARE_API_URL: &str = "https://api.cloudflare.com/client/v4";
const DOMAIN: &str = "neardns.org";

struct CloudflareAPI {
    api_key: String,
    zone_id: String,
    client: Client,
}

#[derive(Deserialize, Debug)]
struct ListDNSResponse {
    result: Vec<DNSResult>,
}

#[derive(Deserialize, Debug)]
struct DNSResult {
    id: String,
    name: String,
    #[serde(rename = "type")]
    record_type: String,
    content: String,
}

impl CloudflareAPI {
    fn new(api_key: String, zone_id: String) -> Self {
        CloudflareAPI {
            api_key,
            zone_id,
            client: Client::new(),
        }
    }

    async fn list_records(self) -> Result<HashMap<String, String>, reqwest::Error> {
        let url = format!("{}/zones/{}/dns_records", CLOUDFLARE_API_URL, self.zone_id);

        let response = self
            .client
            .get(url)
            .header("Content-Type", "application/json")
            .bearer_auth(self.api_key)
            .send()
            .await?;

        let body: ListDNSResponse = response.json().await?;

        println!("Body: {:?}", body);

        let mut records = HashMap::new();
        for r in body.result {
            records.insert(r.name, r.content);
        }

        Ok(records)
    }
}

pub async fn reconcile(domains: Vec<(String, DNSRecord)>) {
    let api_key = std::env::var("CLOUDFLARE_API_KEY").expect("Missing CLOUDFLARE_API_KEY env var");

    let zone_id = std::env::var("CLOUDFLARE_ZONE_ID").expect("Missing CLOUDFLARE_ZONE_ID env var");

    let cloudflare_api = CloudflareAPI::new(api_key, zone_id);
    let existing = cloudflare_api.list_records().await.unwrap();
    println!("Domains in Cloudflare:");
    for (domain, record) in domains {
        println!("{}: {}, {}", domain, record.a, record.aaaa);
        let content = if !record.a.is_empty() {
            record.a.clone()
        } else {
            record.aaaa.clone()
        };
        let record_type = if !record.a.is_empty() { "A" } else { "AAAA" };
        let name = format!("{}.{}", domain, DOMAIN);
        if existing.contains_key(&name) {
            if existing.get(&name).unwrap() == &content {
                continue;
            }
            // Update the content
            println!("Updating record for {}, content {}", name, content);
        } else {
            // Create the record
            println!("Creating record for {}, content {}", name, content);
        }
    }
}
