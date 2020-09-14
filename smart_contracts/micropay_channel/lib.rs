#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract(version = "0.1.0")]
mod micropay_channel {
    use ink_core::storage::{
        HashMap as StorageHashMap,
    };

    #[ink(storage)]
    struct MicropayChannel {
        // (client acct id, server acct id) -> (expiration time, fee, nonce)
        channels: StorageHashMap<(AccountId, AccountId), (u64, Balance, u32)>,
        balances: StorageHashMap<AccountId, Balance>,
    }

    impl MicropayChannel {
        #[ink(constructor)]
        fn new(&mut self) {
        }

        #[ink(message)]
        fn open_channel(&mut self, server: AccountId, expire: u64, fee: Balance) -> bool {
            let from = self.env().caller();
            if from == server {
                return false;
            }
            if let Some(_) = self.channels.get(&(from, server)) {
                return false;
            }
            self.channels.insert((from, server), (expire, fee, 0));
            return true;
        }

        #[ink(message)]
        fn close_channel(&mut self, client: AccountId) -> bool {
            let from = self.env().caller();
            if from == client {
                return false;
            }
            if self.channels.get(&(client, from)) == None {
                return false;
            }
            self.channels.remove(&(client, from));
            return true;
        }

        #[ink(message)]
        fn claim_payment(&mut self, client: AccountId, fee: Balance, nonce: u32) -> bool {
            let from = self.env().caller();
            if from == client {
                return false;
            }
            // todo: verify the claim
            if let Some((_, balance, cur_nonce)) = self.channels.get_mut(&(client, from)) {
                if nonce < *cur_nonce {
                    return false;
                }
                *cur_nonce = nonce + 1;
                let cur_balance = self.balances.get(&from).unwrap_or(&0);
                if fee > *balance {
                    self.balances.insert(from, cur_balance + *balance);
                    *balance = 0;
                } else {
                    self.balances.insert(from, cur_balance + fee);
                    *balance -= fee;
                }
                return true;
            }
            return false;
        }

        #[ink(message)]
        fn get_balance(&mut self, client: AccountId) -> Balance {
            let balance = self.balances.get(&client).unwrap_or(&0);
            return *balance;
        }
    }
}
