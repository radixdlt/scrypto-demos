use scrypto::prelude::*;

#[blueprint]
mod token_sale {
    struct TokenSale {
        // The vault where the UsefulTokens will be stored.
        useful_tokens_vault: Vault,

        // The vault where the xrd payments will be stored.
        xrd_tokens_vault: Vault,

        // The price of a single UsefulToken.
        price_per_token: Decimal
    }

    impl TokenSale {
        pub fn instantiate_token_sale(price_per_token: Decimal) -> ComponentAddress {
            // Creating a new token called "UsefulToken"
            let my_bucket: Bucket = ResourceBuilder::new_fungible()
                .metadata("name", "UsefulToken")
                .metadata("symbol", "USEFUL")
                .mint_initial_supply(1000);

            Self {
                useful_tokens_vault: Vault::with_bucket(my_bucket),
                xrd_tokens_vault: Vault::new(RADIX_TOKEN),
                price_per_token: price_per_token
                
            }
            .instantiate()
            .globalize()
        }

        pub fn buy(&mut self, funds: Bucket) -> Bucket {
            let purchase_amount: Decimal = funds.amount() / self.price_per_token;
            self.xrd_tokens_vault.put(funds);
            self.useful_tokens_vault.take(purchase_amount)
        }
    }
}