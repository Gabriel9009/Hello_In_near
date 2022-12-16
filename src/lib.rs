use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{log, near_bindgen};
near_sdk::setup_alloc!();

const DEFAULT_MESSAGE: &str = "Hello";


#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    message: String,
}


impl Default for Contract {
    fn default() -> Self {
        Self {
            message: DEFAULT_MESSAGE.to_string(),
        }
    }
}


#[near_bindgen]
impl Contract {
    
    pub fn get_greeting(&self) -> String {
        return self.message.clone();
    }

    
    pub fn set_greeting(&mut self, message: String) {
        
        log!("Saving greeting {}", message);
        self.message = message;
    }

    pub fn delete(&mut self, message: String) {
        log!("delete");
        self.message = "".to_string();
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};

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
            epoch_height: 0,
        }
    }
    #[test]
    fn get_default_greeting() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let contract = Contract::default();
        assert_eq!(contract.get_greeting(), "Hello".to_string());
    }

    #[test]
    fn set_then_get_greeting() {
        let context = get_context(vec![], true);
        testing_env!(context);
        let mut contract = Contract::default();
        contract.set_greeting("howdy".to_string());
        assert_eq!(contract.get_greeting(), "howdy".to_string());
    }
    #[test]
    fn set_then_delete_greeting() {
        let context = get_context(vec![], true);
        testing_env!(context);
        let mut contract = Contract::default();
        contract.delete("".to_string());
        assert_eq!(contract.get_greeting(), "".to_string());
    }
}
