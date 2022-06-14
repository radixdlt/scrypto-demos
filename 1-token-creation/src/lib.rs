use scrypto::prelude::*;

blueprint! {
    struct TokenSale {
        // The vault where the UsefulTokens will be stored.
        vault: Vault
    }

    impl TokenSale {
        pub fn new() -> ComponentAddress {
            // Creating a new token called "UsefulToken"
            let my_bucket: Bucket = ResourceBuilder::new_fungible()
                .metadata("name", "UsefulToken")
                .metadata("symbol", "UT")
                .initial_supply(1000);

            Self {
                vault: Vault::with_bucket(my_bucket)
            }
            .instantiate()
            .globalize()
        }
    }
}