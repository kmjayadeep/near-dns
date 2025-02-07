use near_sdk::borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::store::IterableMap;
use near_sdk::{env, log, near};

#[derive(BorshDeserialize, BorshSerialize)]
#[near(serializers = [json])]
pub struct DNSRecord {
    pub owner: String,
    pub a: String,
    pub aaaa: String,
}

// Define the contract structure
#[near(contract_state)]
pub struct Contract {
    records: IterableMap<String, DNSRecord>,
}

// Define the default, which automatically initializes the contract
impl Default for Contract {
    fn default() -> Self {
        Self {
            records: IterableMap::new(b"m"),
        }
    }
}

// Implement the contract structure
#[near]
impl Contract {
    // Public method - returns the DNSRecord given the domain name
    pub fn get_domain(&self, domain: String) -> Option<&DNSRecord> {
        self.records.get(&domain)
    }

    // Public method - returns all DNSRecords
    pub fn get_all_domains(&self) -> Vec<(String, &DNSRecord)> {
        self.records.iter().map(|(k, v)| (k.clone(), v)).collect()
    }

    // Public method - registers a domain with an owner and an A/AAAA record
    pub fn register_domain(&mut self, domain: String, a: String, aaaa: String) {
        let owner = env::signer_account_id().to_string();
        log!("Registering domain: {domain} with A: {a} and AAAA: {aaaa} for owner: {owner}");

        let existing = self.records.get(&domain);

        if existing.is_some() && existing.unwrap().owner != env::signer_account_id() {
            env::panic_str("Only owner can update the domain");
        }

        self.records.insert(domain, DNSRecord { owner, a, aaaa });
    }

    // Public method - deletes a domain record
    pub fn delete_domain(&mut self, domain: String) {
        log!("Deleting domain: {domain}");

        let existing = self.records.get(&domain);

        if existing.is_none() || existing.unwrap().owner != env::signer_account_id() {
            env::panic_str("Invalid domain or wrong owner");
        }

        self.records.remove(&domain);
    }
}

/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_then_get_domain() {
        let mut contract = Contract::default();

        let domain = "example".to_string();
        let a = "a".to_string();
        let aaaa = "aaaa".to_string();

        contract.register_domain(domain, a, aaaa);

        let result = contract.get_domain("example".to_string());
        assert_eq!(result.unwrap().a, "a".to_string());
        assert_eq!(result.unwrap().aaaa, "aaaa".to_string());
    }

    #[test]
    fn delete_domain() {
        let mut contract = Contract::default();

        let domain = "example".to_string();
        let a = "a".to_string();
        let aaaa = "aaaa".to_string();

        contract.register_domain(domain, a, aaaa);

        contract.delete_domain("example".to_string());

        let result = contract.get_domain("example".to_string());
        assert!(result.is_none());
    }

    #[test]
    fn get_all_domains() {
        let mut contract = Contract::default();

        let domain = "example".to_string();
        let a = "a".to_string();
        let aaaa = "aaaa".to_string();

        contract.register_domain(domain, a, aaaa);

        let result = contract.get_all_domains();
        result.iter().for_each(|(k, v)| {
            assert_eq!(*k, "example".to_string());
            assert_eq!(v.a, "a".to_string());
            assert_eq!(v.aaaa, "aaaa".to_string());
        });
    }
}
