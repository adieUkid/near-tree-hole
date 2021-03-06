// To conserve gas, efficient serialization is achieved through Borsh (http://borsh.io/)
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::collections::Vector;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{near_bindgen, setup_alloc, BorshStorageKey};

setup_alloc!();

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKeys {
    Secrets,
    Queue,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct TreeHole {
    total: u64,
    secrets: LookupMap<String, Secret>,
    queue: Vector<String>,
}

impl Default for TreeHole {
    fn default() -> Self {
        Self {
            total: 0,
            secrets: LookupMap::new(StorageKeys::Secrets),
            queue: Vector::new(StorageKeys::Queue),
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize, Debug, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Secret {
    id: String,
    name: String,
    content: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct GetSecretOutput {
    count: u64,
    items: Vec<Secret>,
}

const PAGE_SIZE: usize = 10;

#[near_bindgen]
impl TreeHole {
    pub fn add_secret(&mut self, name: String, content: String) {
        assert!(content != "", "invalid content: {}", content);

        let s = Secret {
            id: self.gen_uuid(),
            name,
            content,
        };

        self.secrets.insert(&s.id, &s);
        self.queue.push(&s.id)
    }

    pub fn get_secrets(&self, page_num: usize) -> GetSecretOutput {
        assert!(page_num > 0, "invalid page_num: {}", page_num);
        let ids = self.queue.to_vec();
        let ids: Vec<&String> = ids
            .iter()
            .rev()
            .skip((page_num - 1) * PAGE_SIZE)
            .take(PAGE_SIZE)
            .collect();

        let mut ss = vec![];
        for id in ids {
            let s = self.secrets.get(&id).unwrap();
            ss.push(s);
        }
        GetSecretOutput {
            count: self.queue.len(),
            items: ss,
        }
    }

    fn gen_uuid(&mut self) -> String {
        self.total = self.total + 1;
        format!("{}", self.total)
    }
}

/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 *
 * To run from contract directory:
 * cargo test -- --nocapture
 *
 * From project root, to run in combination with frontend tests:
 * yarn test
 *
 */
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};

    // mock the context for testing, notice "signer_account_id" that was accessed above from env::
    fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
        VMContext {
            current_account_id: "alice_near".to_string(),
            signer_account_id: "bob_near".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "carol_near".to_string(),
            input,
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view,
            output_data_receivers: vec![],
            epoch_height: 19,
        }
    }

    #[test]
    fn add_secret_and_get_secrets() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = TreeHole::default();
        contract.add_secret("alan".to_string(), "alan's content".to_string());
        contract.add_secret("bob".to_string(), "bob's content".to_string());
        let secrets = contract.get_secrets(1);

        println!("{:?}", secrets);
        assert_eq!(2, secrets.count);
        assert_eq!("bob".to_string(), secrets.items[0].name);
        assert_eq!("alan".to_string(), secrets.items[1].name);
        assert_eq!("bob's content".to_string(), secrets.items[0].content);
        assert_eq!("alan's content".to_string(), secrets.items[1].content);
    }
    #[test]
    #[should_panic]
    fn invalid_page_num() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let contract = TreeHole::default();
        let _ = contract.get_secrets(0);
    }
}
