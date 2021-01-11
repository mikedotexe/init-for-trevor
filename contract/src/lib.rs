use near_sdk::{near_bindgen, env, AccountId};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};

#[global_allocator]
static ALLOC: near_sdk::wee_alloc::WeeAlloc = near_sdk::wee_alloc::WeeAlloc::INIT;

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize)]
pub struct Thing {
    // Owner
    o: AccountId,
}

impl Default for Thing {
    fn default() -> Self {
        Thing {
            o: 0.to_string(),
        }
    }
}

#[near_bindgen]
impl Thing {
    // NOTE: This wasn't possible to use #[init] which seems like it doesnt have env available
    #[init]
    pub fn new() -> Self {
        env::log(format!("ID {}", env::signer_account_id()).as_bytes());
        let mut s = Self::default();
        s.o = env::signer_account_id();
        s
    }

    pub fn who(&self) -> AccountId {
        self.o.clone()
    }
}