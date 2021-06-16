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

#[clonable]
#[typetag::serde]
pub trait Artifact: CSO + Clone + Send + Sync {}

#[clonable]
#[typetag::serde]
pub trait Boon: CSO + Clone + Send + Sync {}

#[clonable]
#[typetag::serde]
pub trait Event: CSO + Clone + Send + Sync {
    fn coin_cost(&self) -> usize;
    fn debt_cost(&self) -> usize;
}

#[clonable]
#[typetag::serde]
pub trait Hex: CSO + Clone + Send + Sync {}

#[clonable]
#[typetag::serde]
pub trait Landmark: CSO + Clone + Send + Sync {}

#[clonable]
#[typetag::serde]
pub trait Project: CSO + Clone + Send + Sync {
    fn coin_cost(&self) -> usize;
}

#[clonable]
#[typetag::serde]
pub trait State: CSO + Clone + Send + Sync {}

#[clonable]
#[typetag::serde]
pub trait Way: CSO + Clone + Send + Sync {}
