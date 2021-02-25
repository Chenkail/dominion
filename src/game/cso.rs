//! "Card-shaped objects"

// Need to allow unused variables for default implementations
#![allow(unused_variables)]

use crate::game::{player::Player, card::Card};
use crate::types::*;

pub trait Artifact {

}

pub trait Boon {

}

pub trait Event {

}

pub trait Hex {

}

pub trait Landmark {

}

pub trait Project {

}

pub trait State {

}

pub trait Way {
    fn effects(&self, player: Player, card: Box<dyn Card>, supply: Supply);
}
