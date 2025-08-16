#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod my_erc721 {
    use ink::storage::Mapping;
    use ink::prelude::{string::String, vec::Vec};

    #[ink(storage)]
    pub struct MyErc721 {
        name: String,
        symbol: String,
        owners: Mapping<u32, AccountId>,
        balances: Mapping<AccountId, u32>,
        token_approvals: Mapping<u32, AccountId>,
        operator_approvals: Mapping<(AccountId, AccountId), bool>,
        token_uri: Mapping<u32, String>,
        total_supply: u32,
    }

    impl MyErc721 {
        /// Constructor
        #[ink(constructor)]
        pub fn new(name: String, symbol: String) -> Self {
            Self {
                name,
                symbol,
                owners: Mapping::default(),
                balances: Mapping::default(),
                token_approvals: Mapping::default(),
                operator_approvals: Mapping::default(),
                token_uri: Mapping::default(),
                total_supply: 0,
            }
        }

        /// Mint NFT baru
        #[ink(message)]
        pub fn mint(&mut self, to: AccountId, token_id: u32, uri: String) {
            assert!(self.owners.get(token_id).is_none(), "Token already exists");

            self.owners.insert(token_id, &to);
            let balance = self.balances.get(&to).unwrap_or(0);
            self.balances.insert(&to, &(balance + 1));
            self.token_uri.insert(token_id, &uri);
            self.total_supply += 1;
        }

        /// Burn NFT
        #[ink(message)]
        pub fn burn(&mut self, token_id: u32) {
            let owner = self.owners.get(token_id).expect("Token does not exist");

            self.owners.remove(token_id);
            self.token_uri.remove(token_id);

            let balance = self.balances.get(&owner).unwrap_or(0);
            self.balances.insert(&owner, &(balance - 1));
            self.total_supply -= 1;
        }

        /// Transfer NFT
        #[ink(message)]
        pub fn transfer(&mut self, from: AccountId, to: AccountId, token_id: u32) {
            let owner = self.owners.get(token_id).expect("Token does not exist");
            assert_eq!(owner, from, "Not token owner");

            self.owners.insert(token_id, &to);

            let from_balance = self.balances.get(&from).unwrap_or(0);
            self.balances.insert(&from, &(from_balance - 1));

            let to_balance = self.balances.get(&to).unwrap_or(0);
            self.balances.insert(&to, &(to_balance + 1));
        }

        /// Approve address untuk 1 token
        #[ink(message)]
        pub fn approve(&mut self, to: AccountId, token_id: u32) {
            let owner = self.owners.get(token_id).expect("Token does not exist");
            assert_ne!(to, owner, "Cannot approve self");

            self.token_approvals.insert(token_id, &to);
        }

        /// Set approval for all
        #[ink(message)]
        pub fn set_approval_for_all(&mut self, operator: AccountId, approved: bool) {
            let caller = self.env().caller();
            self.operator_approvals.insert((caller, operator), &approved);
        }

        /// Getter balance
        #[ink(message)]
        pub fn balance_of(&self, owner: AccountId) -> u32 {
            self.balances.get(&owner).unwrap_or(0)
        }

        /// Getter owner of token
        #[ink(message)]
        pub fn owner_of(&self, token_id: u32) -> AccountId {
            self.owners.get(token_id).expect("Token does not exist")
        }

        /// Getter metadata name
        #[ink(message)]
        pub fn name(&self) -> String {
            self.name.clone()
        }

        /// Getter metadata symbol
        #[ink(message)]
        pub fn symbol(&self) -> String {
            self.symbol.clone()
        }

        /// Getter token URI
        #[ink(message)]
        pub fn token_uri(&self, token_id: u32) -> String {
            self.token_uri.get(token_id).unwrap_or(String::from(""))
        }
    }
}
