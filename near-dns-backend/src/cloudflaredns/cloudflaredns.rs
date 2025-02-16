use crate::DNSRecord;
use cloudflare::endpoints::dns;
use cloudflare::framework::{
    response::{ApiFailure, ApiResponse, ApiResult},
    Environment, HttpApiClient, HttpApiClientConfig, OrderDirection,
};

fn dns(zone_identifier: String, api_client: &HttpApiClient) {
    let endpoint = dns::ListDnsRecords {
        zone_identifier: &zone_identifier,
        params: dns::ListDnsRecordsParams {
            direction: Some(OrderDirection::Ascending),
            ..Default::default()
        },
    };
    let response = api_client.request(&endpoint);
    print_response(response);
}

fn print_response<T>(response: ApiResponse<T>)
where
    T: ApiResult,
{
    match response {
        Ok(success) => println!("Success: {success:#?}"),
        Err(e) => match e {
            ApiFailure::Error(status, errors) => {
                println!("HTTP {status}:");
                for err in errors.errors {
                    println!("Error {}: {}", err.code, err.message);
                    for (k, v) in err.other {
                        println!("{k}: {v}");
                    }
                }
                for (k, v) in errors.other {
                    println!("{k}: {v}");
                }
            }
            ApiFailure::Invalid(reqwest_err) => println!("Error: {reqwest_err}"),
        },
    }
}

pub fn reconcile(_domains: Vec<(String, DNSRecord)>) {
    let api_key = std::env::var("CLOUDFLARE_API_KEY").expect("Missing CLOUDFLARE_API_KEY env var");

    let zone_id = std::env::var("CLOUDFLARE_ZONE_ID").expect("Missing CLOUDFLARE_ZONE_ID env var");

    println!("Domains in Cloudflare:");
    let api_client = HttpApiClient::new(
        cloudflare::framework::auth::Credentials::UserAuthToken { token: api_key },
        HttpApiClientConfig::default(),
        Environment::Production,
    )
    .unwrap();
    dns(String::from(zone_id), &api_client);
}
