//! This module implements the [ZombieFactory] blueprint and all associated types. This blueprint
//! has the required knowledge to create new zombies.

use crate::random::*;
use crate::zombie::*;
use scrypto::prelude::*;

blueprint! {
    /// A structure that defines the type of the state held by `ZombieFactory` components.
    ///
    /// The main responsibility and job of a zombie factory is the creation of more zombie NFTs. In
    /// particular, this factory has knowledge of how to create new zombie DNAs, how to map the
    /// zombie DNAs to the actual zombie traits, and then how to mint these zombie NFTs and return
    /// them back to the caller.
    struct ZombieFactory {
        /// The [ResourceAddress] of the zombies' NFT resource.
        zombie_resource_address: ResourceAddress,

        /// A [Vault] containing the admin badge. This badge has the authority to mint additional
        /// zombies.
        internal_admin_badge: Vault
    }

    impl ZombieFactory {
        pub fn instantiate_zombie_factory() -> ComponentAddress {
            // This function needs to create two resources. One resource will be the admin badge
            // whose main job would be to authorize the minting of zombie NFTs, and another resource
            // which is the NFT itself.

            // Creating a new fungible resource (token) which we will be using as an internal admin
            // badge.
            let internal_admin_badge: Bucket = ResourceBuilder::new_fungible()
                .metadata("name", "Admin Badge")
                .metadata("description", "A badge which has the authority to mint zombie tokens")
                .initial_supply(1);

            // Creating a new non-fungible resource (token) which will be used as an on-chain
            // representation of the zombies.
            let zombie_resource: ResourceAddress = ResourceBuilder::new_non_fungible(NonFungibleIdType::UUID)
                .metadata("name", "Crypto Zombies")
                .metadata("description", "An NFT of a Crypto Zombie!")
                .mintable(rule!(require(internal_admin_badge.resource_address())), LOCKED)
                .no_initial_supply();

            // Initializing and creating the component.
            Self {
                zombie_resource_address: zombie_resource,
                internal_admin_badge: Vault::with_bucket(internal_admin_badge)
            }
            .instantiate()
            .globalize()
        }

        /// Creates a new **pseudo** random zombie.
        ///
        /// This function creates a new **pseudo** random zombie through the **pseudo** random
        /// number generators defined in [`crate::random`] and returns an NFT of the zombie in
        /// a bucket.
        pub fn new_random_zombie(&self) -> Bucket {
            let head_item: HeadItem = pseudo_random_enum_variant();
            let eye_item: EyeItem = pseudo_random_enum_variant();
            let shirt_item: ShirtItem = pseudo_random_enum_variant();

            let skin_color: Color = pseudo_random_number(360) as u16;
            let eye_color: Color = pseudo_random_number(360) as u16;
            let clothes_color: Color = pseudo_random_number(360) as u16;

            let zombie: Zombie = Zombie::new(
                head_item,
                eye_item,
                shirt_item,
                skin_color,
                eye_color,
                clothes_color,
            );
            self.internal_admin_badge.authorize(|| {
                borrow_resource_manager!(self.zombie_resource_address)
                    .mint_non_fungible(&NonFungibleId::random(), zombie)
            })
        }
    }
}
