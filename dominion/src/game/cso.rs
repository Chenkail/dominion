//! "Card-shaped objects"

// Need to allow unused variables for default implementations
#![allow(unused_variables)]

use dyn_clonable::clonable;

use crate::game::{player::Player, card::Card};
use crate::types::*;

#[clonable]
#[typetag::serde(tag = "cso")]
pub trait CSO: Clone + Send + Sync {
    fn effects(&self, player: Player, card: Box<dyn Card>, supply: Supply);
}

#[typetag::serde]
pub trait Artifact: CSO {}

#[typetag::serde]
pub trait Boon: CSO {}

#[typetag::serde]
pub trait Event: CSO {
    fn coin_cost(&self) -> usize;
    fn debt_cost(&self) -> usize;
}

#[typetag::serde]
pub trait Hex: CSO {}

#[typetag::serde]
pub trait Landmark: CSO {}

#[typetag::serde]
pub trait Project: CSO {
    fn coin_cost(&self) -> usize;
}

#[typetag::serde]
pub trait State: CSO {}

#[typetag::serde]
pub trait Way: CSO {}
