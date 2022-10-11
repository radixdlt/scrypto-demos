//! This module implements the [ZombieFactory] blueprint and all associated types. This blueprint
//! has the required knowledge to create new zombies.

use scrypto::prelude::*;
use crate::random::*;
use crate::zombie::*;

blueprint! {
    struct ZombieFactory {}

    impl ZombieFactory {}
}
