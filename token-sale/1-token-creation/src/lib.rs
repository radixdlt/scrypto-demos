use scrypto::prelude::*;

#[blueprint]
mod token_sale {
    struct TokenSale {
        // The vault where the UsefulTokens will be stored.
        useful_tokens_vault: Vault
    }

    impl TokenSale {
        pub fn instantiate_token_sale() -> ComponentAddress {
            // Creating a new token called "UsefulToken"
            let my_bucket: Bucket = ResourceBuilder::new_fungible()
                .metadata("name", "UsefulToken")
                .metadata("symbol", "USEFUL")
                .mint_initial_supply(1000);

            Self {
                useful_tokens_vault: Vault::with_bucket(my_bucket)
            }
            .instantiate()
            .globalize()
        }
    }
}