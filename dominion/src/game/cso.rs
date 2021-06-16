//! "Card-shaped objects"

// Need to allow unused variables for default implementations
#![allow(unused_variables)]

use crate::game::{player::Player, card::Card};
use crate::types::*;

pub trait CSO {
    fn effects(&self, player: Player, card: Box<dyn Card>, supply: Supply);
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub trait Artifact: CSO {}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub trait Boon: CSO {}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub trait Event: CSO {
    fn coin_cost(&self) -> usize;
    fn debt_cost(&self) -> usize;
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub trait Hex: CSO {}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub trait Landmark: CSO {}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub trait Project: CSO {
    fn coin_cost(&self) -> usize;
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub trait State: CSO {}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub trait Way: CSO {}
