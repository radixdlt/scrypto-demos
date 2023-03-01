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
        pub fn instantiate_token_sale(price_per_token: Decimal) -> (ComponentAddress, Bucket) {

            // Creating a new token called "UsefulToken"
            let my_bucket: Bucket = ResourceBuilder::new_fungible()
                .metadata("name", "UsefulToken")
                .metadata("symbol", "USEFUL")
                .mint_initial_supply(1000);

            // Creating a new seller badge which we will give the withdraw authority to
            let seller_badge: Bucket = ResourceBuilder::new_fungible()
                .metadata("name", "Seller Badge")
                .metadata("symbol", "SELLER")
                .mint_initial_supply(1);

            // Setting the access rules to only allow the seller badge to withdraw the funds or change the price
            let access_rules: AccessRules = AccessRules::new()
                .method("withdraw_funds", rule!(require(seller_badge.resource_address())), LOCKED)
                .method("change_price", rule!(require(seller_badge.resource_address())), LOCKED)
                .default(rule!(allow_all), LOCKED);

            let mut token_sale: TokenSaleComponent = Self {
                useful_tokens_vault: Vault::with_bucket(my_bucket),
                xrd_tokens_vault: Vault::new(RADIX_TOKEN),
                price_per_token: price_per_token
            }
            .instantiate();
            token_sale.add_access_check(access_rules);
            let token_sale_component_address: ComponentAddress = token_sale.globalize();

            return (token_sale_component_address, seller_badge)
        }

        pub fn buy(&mut self, funds: Bucket) -> Bucket {
            let purchase_amount: Decimal = funds.amount() / self.price_per_token;
            self.xrd_tokens_vault.put(funds);
            self.useful_tokens_vault.take(purchase_amount)
        }

        pub fn withdraw_funds(&mut self, amount: Decimal) -> Bucket {
            self.xrd_tokens_vault.take(amount)
        }

        pub fn change_price(&mut self, price: Decimal) {
            self.price_per_token = price;
        }
    }
}