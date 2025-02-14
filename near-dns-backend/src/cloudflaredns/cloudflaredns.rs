use crate::DNSRecord;
use cloudflare::endpoints::dns;
use cloudflare::framework::{
    response::{ApiFailure, ApiResponse, ApiResult},
    HttpApiClient, HttpApiClientConfig, OrderDirection, Environment,
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

pub fn reconcile(domains: Vec<(String, DNSRecord)>) {
    println!("Domains in Cloudflare:");
    let api_client = HttpApiClient::new(
        cloudflare::framework::auth::Credentials::UserAuthKey {
            key: "your_api_key".to_string(),
            email: "your_email@example.com".to_string(),
        },
        HttpApiClientConfig::default(),
        Environment::Production,
    )
    .unwrap();
    dns(String::from("neardns.org"), &api_client);
}