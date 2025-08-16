#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod my_erc721 {
    use ink::prelude::string::String;

    #[ink(storage)]
    pub struct MyErc721 {
        value: String,
    }

    impl MyErc721 {
        #[ink(constructor)]
        pub fn new(init_value: String) -> Self {
            Self { value: init_value }
        }

        #[ink(message)]
        pub fn get(&self) -> String {
            self.value.clone()
        }
    }
}
