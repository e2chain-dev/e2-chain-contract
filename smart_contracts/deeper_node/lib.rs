#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract(version = "0.1.0")]
mod deeper_node {
    use ink_core::storage::{
        HashMap as StorageHashMap,
    };

    #[ink(storage)]
    struct DeeperNode {
        nodes: StorageHashMap<AccountId, (u32, u16, bool)>,
        server_cnt: StorageHashMap<u16, u32>,
    }

    #[ink(event)]
    struct RegisterNode {
        #[ink(topic)]
        node_id: AccountId,
        #[ink(topic)]
        ip_addr: u32,
        #[ink(topic)]
        country: u16,
        #[ink(topic)]
        is_server: bool,
    }

    #[ink(event)]
    struct UnRegisterNode {
        #[ink(topic)]
        node_id: AccountId,
        #[ink(topic)]
        ip_addr: u32,
        #[ink(topic)]
        country: u16,
        #[ink(topic)]
        is_server: bool,
    }

    #[ink(event)]
    struct GetNodeInfo {
        #[ink(topic)]
        node_id: AccountId,
        #[ink(topic)]
        ip_addr: u32,
        #[ink(topic)]
        country: u16,
        #[ink(topic)]
        is_server: bool,
    }

    impl DeeperNode {
        #[ink(constructor)]
        fn new(&mut self) {
        }

        #[ink(message)]
        fn register_node(&mut self, ip_addr: u32, country: u16, is_server: bool) {
            self.unregister_node();
            let from = self.env().caller();
            self.nodes.insert(from, (ip_addr, country, is_server));
            if is_server == true {
                let server_cnt = self.server_cnt.get(&country).unwrap_or(&0);
                self.server_cnt.insert(country, server_cnt + 1);
            }
            self.env().emit_event(RegisterNode {
                node_id: from,
                ip_addr: ip_addr,
                country: country,
                is_server: is_server,
            });
        }

        #[ink(message)]
        fn unregister_node(&mut self) {
            let from = self.env().caller();
            if let Some((ip_addr, country, is_server)) = self.nodes.get(&from) {
                self.env().emit_event(UnRegisterNode {
                    node_id: from,
                    ip_addr: *ip_addr,
                    country: *country,
                    is_server: *is_server,
                });

                if let Some(server_cnt) = self.server_cnt.get_mut(&country) {
                    *server_cnt -= 1;
                }
                self.nodes.remove(&from);
            }
        }

        #[ink(message)]
        fn get_node_info(&self, acct_id: AccountId) {
            if let Some((ip_addr, country, is_server)) = self.nodes.get(&acct_id) {
                self.env().emit_event(GetNodeInfo {
                    node_id: acct_id,
                    ip_addr: *ip_addr,
                    country: *country,
                    is_server: *is_server,
                });
            }
        }

        #[ink(message)]
        fn get_server_cnt(&self, country: u16) -> u32 {
            let server_cnt = self.server_cnt.get(&country).unwrap_or(&0);
            *server_cnt
        }
    }
}
