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
    greeting: String,
    records: IterableMap<String, DNSRecord>,
}

// Define the default, which automatically initializes the contract
impl Default for Contract {
    fn default() -> Self {
        Self {
            greeting: "Hello".to_string(),
            records: IterableMap::new(b"m"),
        }
    }
}

// Implement the contract structure
#[near]
impl Contract {
    // Public method - returns the greeting saved, defaulting to DEFAULT_GREETING
    pub fn get_greeting(&self) -> String {
        self.greeting.clone()
    }

    // Public method - returns the DNSRecord given the domain name
    pub fn get_domain(&self, domain: String) -> Option<&DNSRecord> {
        self.records.get(&domain)
    }

    // Public method - accepts a greeting, such as "howdy", and records it
    pub fn set_greeting(&mut self, greeting: String) {
        log!("Saving greeting: {greeting}");
        self.greeting = greeting;
    }

    // Public method - registers a domain with an owner and an A/AAAA record
    pub fn register_domain(&mut self, domain: String, a: String, aaaa: String) {
        log!("Registering domain: {domain} with A: {a} and AAAA: {aaaa}");
        let owner = env::signer_account_id().to_string();

        let existing = self.records.get(&domain);

        if existing.is_some() && existing.unwrap().owner != env::signer_account_id() {
            env::panic_str("Only owner can update the domain");
        }

        self.records.insert(domain, DNSRecord { owner, a, aaaa });
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
    fn get_default_greeting() {
        let contract = Contract::default();
        // this test did not call set_greeting so should return the default "Hello" greeting
        assert_eq!(contract.get_greeting(), "Hello");
    }

    #[test]
    fn set_then_get_greeting() {
        let mut contract = Contract::default();
        contract.set_greeting("howdy".to_string());
        assert_eq!(contract.get_greeting(), "howdy");
    }
}
