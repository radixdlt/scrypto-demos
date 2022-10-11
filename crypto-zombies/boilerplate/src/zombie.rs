//! This module defines all of the requires types and type aliases required for the [Zombie] struct.
//! This struct defines the data that each zombie NFT will have.

use scrypto::prelude::*;
use strum_macros::EnumIter; 

/// A struct that defines the data that an individual zombie has.
///
/// This struct derives the [`NonFungibleData`] trait which means that this struct can be used as 
/// the data for a non-fungible token. This is something that we need to have since we intend for
/// zombies to eventually be NFTs.
#[derive(NonFungibleData)]
pub struct Zombie {
    /// The current equipped head item
    pub head: HeadItem,
    /// The current equipped eye item
    pub eye: EyeItem,
    /// The current equipped shirt
    pub shirt: ShirtItem,
    /// The current equipped skin color
    pub skin_color: Color,
    /// The current equipped eye color
    pub eye_color: Color,
    /// The current equipped clothes color
    pub clothes_color: Color,
}

impl Zombie {
    pub fn new(
        head: HeadItem,
        eye: EyeItem,
        shirt: ShirtItem,
        skin_color: Color,
        eye_color: Color,
        clothes_color: Color,
    ) -> Self {
        Self {
            head,
            eye,
            shirt,
            skin_color,
            eye_color,
            clothes_color,
            
        }
    }
}

/// An enum defining the possible items that a zombie can have on their head
#[derive(Debug, Copy, Clone, TypeId, Encode, Decode, Describe, EnumIter, PartialEq, Eq, PartialOrd, Ord)]
pub enum HeadItem {
    ArmyHat,
    PonyTail,
    GeneralHat,
    Bangs,
    LongHairAndBangs,
    BlackLongHair,
    ChristmasHat,
}

/// An enum defining the possible eyes that a zombie can have
#[derive(Debug, Copy, Clone, TypeId, Encode, Decode, Describe, EnumIter, PartialEq, Eq, PartialOrd, Ord)]
pub enum EyeItem {
    DefaultEyes,
    EyesWithGlasses,
    LazyEyes,
    LazyEyesFalling,
    SmallEyes,
    PunchedEyes,
    AngryEyes,
    EyeWithEyeLashes,
    BlackEye,
    SuspiciousEyes,
    SleepyEyesWithEyeLashes,
}

/// An enum describing the possible shirts that a zombie can have.
#[derive(Debug, Copy, Clone, TypeId, Encode, Decode, Describe, EnumIter, PartialEq, Eq, PartialOrd, Ord)]
pub enum ShirtItem {
    Classic,
    White,
    Blue,
    BloodyWhite,
    Red,
    WhiteButtonDown,
}

/// A type representing a number between 0 and 360.
pub type Color = u16;
