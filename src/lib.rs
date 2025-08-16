#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod my_erc721 {
    use ink::prelude::string::String;
    use ink::prelude::vec::Vec;
    use ink::storage::Mapping;

    #[ink(storage)]
    pub struct MyErc721 {
        owner: AccountId,
        name: String,
        symbol: String,
        token_owner: Mapping<u32, AccountId>,
        balances: Mapping<AccountId, u32>,
        token_uris: Mapping<u32, String>,
        total_supply: u32,
    }

    impl MyErc721 {
        /// Constructor: buat NFT baru dengan `name` dan `symbol`
        #[ink(constructor)]
        pub fn new(name: String, symbol: String) -> Self {
            let caller = Self::env().caller();
            Self {
                owner: caller,
                name,
                symbol,
                token_owner: Mapping::default(),
                balances: Mapping::default(),
                token_uris: Mapping::default(),
                total_supply: 0,
            }
        }

        /// Mint NFT baru ke `to` dengan metadata `token_uri`
        #[ink(message)]
        pub fn mint(&mut self, to: AccountId, token_uri: String) {
            let caller = self.env().caller();
            assert_eq!(caller, self.owner, "Only owner can mint");

            let token_id = self.total_supply + 1;
            self.token_owner.insert(token_id, &to);

            let balance = self.balance_of(to) + 1;
            self.balances.insert(to, &balance);

            self.token_uris.insert(token_id, &token_uri);

            self.total_supply += 1;
        }

        /// Transfer NFT dari `from` ke `to`
        #[ink(message)]
        pub fn transfer(&mut self, from: AccountId, to: AccountId, token_id: u32) {
            let caller = self.env().caller();
            let owner = self.owner_of(token_id);
            assert_eq!(caller, owner, "Caller is not token owner");
            assert_eq!(from, owner, "From address is not token owner");

            self.token_owner.insert(token_id, &to);

            let from_balance = self.balance_of(from) - 1;
            self.balances.insert(from, &from_balance);

            let to_balance = self.balance_of(to) + 1;
            self.balances.insert(to, &to_balance);
        }

        /// Cek pemilik NFT berdasarkan `token_id`
        #[ink(message)]
        pub fn owner_of(&self, token_id: u32) -> AccountId {
            self.token_owner
                .get(token_id)
                .expect("Token does not exist")
        }

        /// Cek jumlah NFT yang dimiliki `owner`
        #[ink(message)]
        pub fn balance_of(&self, owner: AccountId) -> u32 {
            self.balances.get(owner).unwrap_or(0)
        }

        /// Ambil metadata/URI NFT
        #[ink(message)]
        pub fn token_uri(&self, token_id: u32) -> String {
            self.token_uris
                .get(token_id)
                .expect("Token does not exist")
        }

        /// Total supply NFT
        #[ink(message)]
        pub fn total_supply(&self) -> u32 {
            self.total_supply
        }

        /// Ambil nama koleksi
        #[ink(message)]
        pub fn name(&self) -> String {
            self.name.clone()
        }

        /// Ambil simbol koleksi
        #[ink(message)]
        pub fn symbol(&self) -> String {
            self.symbol.clone()
        }
    }
      }
      
