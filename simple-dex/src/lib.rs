use scrypto::prelude::*;

blueprint! {
    struct Radiswap {
        /// A vault containing pool reverses of reserves of token A.
        vault_a: Vault,
        /// A vault containing pool reverses of reserves of token B.
        vault_b: Vault,

        /// The token address of a token representing pool shares in this pool
        pool_share_resource_address: ResourceAddress,
        /// A vault containing a badge which has the authority to mint `pool_share` tokens.
        pool_share_minter_badge: Vault,

        /// The amount of fees imposed by the pool on swaps where 0 <= fee <= 1.
        fee: Decimal,
    }

    impl Radiswap {
        /// Creates a new liquidity pool of the two tokens sent to the pool
        pub fn instantiate_pool(
            bucket_a: Bucket,
            bucket_b: Bucket,

            pool_shares_initial_supply: Decimal,

            fee: Decimal,
        ) -> (ComponentAddress, Bucket) {
            // Ensure that none of the buckets are empty and that an appropriate fee is set.
            assert!(
                !bucket_a.is_empty() && !bucket_b.is_empty(),
                "You must pass in an initial supply of each token"
            );
            assert!(
                fee >= dec!("0") && fee <= dec!("1"),
                "Invalid fee in thousandths"
            );

            // Create a badge which will be given the authority to mint the pool share tokens.
            let pool_share_minter_badge: Bucket = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", "LP Token Mint Auth")
                .initial_supply(1);

            // Create the pool share resource along with the initial supply specified by the user.
            let initial_pool_shares: Bucket = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_MAXIMUM)
                .metadata("name", "Pool Share")
                .metadata("symbol", "SHARE")
                .mintable(
                    rule!(require(pool_share_minter_badge.resource_address())),
                    LOCKED,
                )
                .burnable(
                    rule!(require(pool_share_minter_badge.resource_address())),
                    LOCKED,
                )
                .initial_supply(pool_shares_initial_supply);

            // Create the Radiswap component and globalize it
            let radiswap: ComponentAddress = Self {
                vault_a: Vault::with_bucket(bucket_a),
                vault_b: Vault::with_bucket(bucket_b),

                pool_share_resource_address: initial_pool_shares.resource_address(),
                pool_share_minter_badge: Vault::with_bucket(pool_share_minter_badge),

                fee,
            }
            .instantiate()
            .globalize();

            // Return the component address as well as the pool share tokens
            (radiswap, initial_pool_shares)
        }

        /// Swaps token A for B, or vice versa.
        pub fn swap(&mut self, input_tokens: Bucket) -> Bucket {
            // Getting the vault corresponding to the input tokens and the vault corresponding to the output tokens
            // based on what the input is.
            let (input_tokens_vault, output_tokens_vault): (&mut Vault, &mut Vault) =
                if input_tokens.resource_address() == self.vault_a.resource_address() {
                    (&mut self.vault_a, &mut self.vault_b)
                } else if input_tokens.resource_address() == self.vault_b.resource_address() {
                    (&mut self.vault_b, &mut self.vault_a)
                } else {
                    panic!("The given input tokens do not belong to this liquidity pool")
                };

            // Calculate the output amount of tokens based on the input amount and the pool fees
            let output_amount: Decimal = (input_tokens.amount()
                * (dec!("1") - self.fee)
                * output_tokens_vault.amount())
                / (input_tokens_vault.amount() + input_tokens.amount() * (dec!("1") - self.fee));

            // Perform the swapping operation
            input_tokens_vault.put(input_tokens);
            output_tokens_vault.take(output_amount)
        }

        /// Removes the amount of funds from the pool corresponding to the pool shares.
        pub fn remove_liquidity(&mut self, pool_shares: Bucket) -> (Bucket, Bucket) {
            assert!(
                self.pool_share_resource_address == pool_shares.resource_address(),
                "Wrong token type passed in"
            );

            // Get the resource manager of the lp tokens
            let pool_share_resource_manager: &ResourceManager =
                borrow_resource_manager!(self.pool_share_resource_address);

            // Calculate the share based on the input LP tokens.
            let share = pool_shares.amount() / pool_share_resource_manager.total_supply();

            // Burn the LP tokens received
            self.pool_share_minter_badge.authorize(|| {
                pool_shares.burn();
            });

            // Return the withdrawn tokens
            (
                self.vault_a.take(self.vault_a.amount() * share),
                self.vault_b.take(self.vault_b.amount() * share),
            )
        }

        /// Adds liquidity to the liquidity pool
        pub fn add_liquidity(
            &mut self,
            bucket_a: Bucket,
            bucket_b: Bucket,
        ) -> (Bucket, Bucket, Bucket) {
            // Give the buckets the same names as the vaults
            let (mut bucket_a, mut bucket_b): (Bucket, Bucket) = if bucket_a.resource_address()
                == self.vault_a.resource_address()
                && bucket_b.resource_address() == self.vault_b.resource_address()
            {
                (bucket_a, bucket_b)
            } else if bucket_a.resource_address() == self.vault_b.resource_address()
                && bucket_b.resource_address() == self.vault_a.resource_address()
            {
                (bucket_b, bucket_a)
            } else {
                panic!("One of the tokens does not belong to the pool!")
            };

            // Getting the values of `dm` and `dn` based on the sorted buckets
            let dm: Decimal = bucket_a.amount();
            let dn: Decimal = bucket_b.amount();

            // Getting the values of m and n from the liquidity pool vaults
            let m: Decimal = self.vault_a.amount();
            let n: Decimal = self.vault_b.amount();

            // Calculate the amount of tokens which will be added to each one of the vaults
            let (amount_a, amount_b): (Decimal, Decimal) =
                if ((m == Decimal::zero()) | (n == Decimal::zero())) | ((m / n) == (dm / dn)) {
                    // Case 1
                    (dm, dn)
                } else if (m / n) < (dm / dn) {
                    // Case 2
                    (dn * m / n, dn)
                } else {
                    // Case 3
                    (dm, dm * n / m)
                };

            // Depositing the amount of tokens calculated into the liquidity pool
            self.vault_a.put(bucket_a.take(amount_a));
            self.vault_b.put(bucket_b.take(amount_b));

            // Mint pool share tokens to the liquidity provider
            let tracking_tokens_manager: &ResourceManager =
                borrow_resource_manager!(self.pool_share_resource_address);
            let tracking_amount: Decimal =
                if tracking_tokens_manager.total_supply() == Decimal::zero() {
                    dec!("100.00")
                } else {
                    amount_a * tracking_tokens_manager.total_supply() / m
                };
            let tracking_tokens: Bucket = self
                .pool_share_minter_badge
                .authorize(|| tracking_tokens_manager.mint(tracking_amount));

            // Return the remaining tokens to the caller as well as the pool share tokens
            (bucket_a, bucket_b, tracking_tokens)
        }
    }
}
