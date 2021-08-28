// To conserve gas, efficient serialization is achieved through Borsh (http://borsh.io/)
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::collections::Vector;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{near_bindgen, setup_alloc};
use uuid::Uuid;

setup_alloc!();

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct TreeHole {
    secrets: LookupMap<String, Secret>,
    queue: Vector<String>,
}

impl Default for TreeHole {
    fn default() -> Self {
        Self {
            secrets: LookupMap::new(b"s".to_vec()),
            queue: Vector::new(b"q".to_vec()),
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

const PAGE_SIZE: usize = 10;

#[near_bindgen]
impl TreeHole {
    pub fn add_secret(&mut self, name: String, content: String) {
        let my_uuid = Uuid::new_v4();
        let s = Secret {
            id: my_uuid.to_string(),
            name,
            content,
        };

        self.secrets.insert(&s.id, &s);
        self.queue.push(&s.id)
    }

    pub fn get_secrets(&self, page_num: usize) -> Vec<Secret> {
        assert!(page_num > 0, "invalid page_num: {}", page_num);
        let ids: Vec<String> = self
            .queue
            .iter()
            .skip((page_num - 1) * PAGE_SIZE)
            .take(PAGE_SIZE)
            .collect();

        let mut ss = vec![];
        for id in ids {
            let s = self.secrets.get(&id).unwrap();
            ss.push(s);
        }
        ss
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
    fn add_secret_and_get_it() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = TreeHole::default();
        contract.add_secret("alan".to_string(), "test content".to_string());
        let id = contract.queue.get(0).unwrap();
        let s = contract.secrets.get(&id).unwrap();
        assert_eq!("alan".to_string(), s.name);
        assert_eq!("test content".to_string(), s.content);
    }

    #[test]
    fn add_secret_and_get_secrets() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = TreeHole::default();
        contract.add_secret("alan".to_string(), "test content".to_string());
        let secrets = contract.get_secrets(1);

        println!("{:?}", secrets);
        assert_eq!("alan".to_string(), secrets[0].name);
        assert_eq!("test content".to_string(), secrets[0].content);
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
