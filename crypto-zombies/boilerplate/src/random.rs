//! This module includes a number of functions used to generate random numbers. This module makes 
//! assumptions about the runtime being a Scrypto runtime with a way to communicate to the Radix
//! Engine to generate UUIDs.

use strum::IntoEnumIterator;
use scrypto::core::Runtime;

/// Generates a **pseudo** random variant of an enum.
/// 
/// This function generates a random variant for any enum that implements the [`IntoEnumIterator`] 
/// and [`Clone`] traits. 
pub fn pseudo_random_enum_variant<T: IntoEnumIterator + Clone>() -> T {
    pseudo_random_choose(&T::iter().collect::<Vec<_>>())
}

/// Generates a **pseudo** random number between 0 and `to_inclusive`.
/// 
/// This function generates a **pseudo** random number between 0 and `to_inclusive` by using the
/// Radix Engine's UUID generator to generate random unique identifiers.
/// 
/// # Panics
/// 
/// This function panics if the host is unable to provide UUIDs (either because the host is not the
/// Radix Engine, or for any other reason.)
pub fn pseudo_random_number(to_inclusive: u128) -> u128 {
    // Creating a new pseudo random number through Scrypto's uuid generator.
    let pseudo_random_number: u128 = Runtime::generate_uuid();

    // Ensure that then number is in the range specified by the `to_inclusive`
    pseudo_random_number % (to_inclusive + 1)
}

/// Makes a random selection from a list of items
/// 
/// This function uses the [`pseudo_random_number`] function to generate a random choice of one of 
/// the items in the given list of items.
pub fn pseudo_random_choose<T: Clone>(array: &[T]) -> T {
    // Return the value at the index specified by the pseudo random number.
    array
        .get(pseudo_random_number(array.len() as u128 - 1) as usize)
        .expect("Value at a trusted index can not fail")
        .clone()
}
