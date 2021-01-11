//! Defines traits for the various card types

use crate::game::{gamedata::Game, player::Player};
use std::hash::{Hash, Hasher};

/// The basic Card type
///
/// dyn Card implements Hash and Eq so that Box\<dyn Card\> can be used as keys for a HashMap
pub trait Card {
    fn cost(&self) -> i32;
    fn name(&self) -> &'static str;

    fn description(&self) -> &'static str {
        return "";
    }
}

impl Hash for dyn Card {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name().hash(state);
    }
}

impl PartialEq for dyn Card {
    fn eq(&self, other: &Self) -> bool {
        self.name().eq(other.name())
    }
}

impl Eq for dyn Card {}

pub trait Treasure: Card {
    fn value(&self, player: &Player) -> i32;
}

pub trait Victory: Card {
    fn points(&self, player: &Player) -> i32;
}

pub trait Action: Card {
    /// Effects that the Action card has on the person playing it
    fn effects(&self, player: &mut Player, game: &mut Game);
}

pub trait Attack: Action {
    /// Effects that the Attack card has
    /// TODO: Figure out how to implement attack cards that hit everyone - boolean flag?
    fn attack(&self, player: &mut Player, target: &mut Player);
}

pub trait Reaction: Action {
    /// Effects that the Reaction card has
    /// TODO: player boolean flag for Moat immunity?
    fn react(&self, player: &mut Player);
}

pub (crate) trait CurseTrait: Card {
    fn points(&self, player: &Player) -> i32;
}

