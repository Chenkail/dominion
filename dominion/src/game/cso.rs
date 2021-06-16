//! "Card-shaped objects"

// Need to allow unused variables for default implementations
#![allow(unused_variables)]

use crate::game::{player::Player, card::Card};
use crate::types::*;

pub trait CSO {
    fn effects(&self, player: Player, card: Box<dyn Card>, supply: Supply);
}

pub trait Artifact: CSO {}

pub trait Boon: CSO {}

pub trait Event: CSO {
    fn coin_cost(&self) -> usize;
    fn debt_cost(&self) -> usize;
}

pub trait Hex: CSO {}

pub trait Landmark: CSO {}

pub trait Project: CSO {
    fn coin_cost(&self) -> usize;
}

pub trait State: CSO {}

pub trait Way: CSO {}
