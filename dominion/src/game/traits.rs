//! Defines traits for the various card types

use std::hash::{Hash, Hasher};
use crate::game::{gamedata::Game, player::Player};

/// The basic Card trait
///
/// dyn Card implements [Hash] and [Eq] so that Box\<dyn Card\> can be used as keys for a HashMap
pub trait Card {
    /// How much the card costs to buy
    fn cost(&self) -> i32;
    /// The name on the card (e.g. "Throne Room")
    fn name(&self) -> &str;
    /// The card text (this will often be blank, as is the case with all the cards in the base set)
    fn description(&self) -> &str {
        return "";
    }
    fn types(&self) -> Vec<&str>;

    fn treasure_value(&self, _: &Player) -> i32 { 0 }
    fn victory_points(&self, _: &Player) -> i32 { 0 }
    fn curse_points(&self, _: &Player) -> i32 { 0 }
    fn action_effects(&self, _: &mut Player, _: &mut Game) {}
    fn attack_effects(&self, _: &mut Player, _: &mut Game) {}
    fn reaction_effects(&self, _: &mut Player, _: &mut Game) {}
}

impl Hash for dyn Card {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name().hash(state);
    }
}

impl PartialEq for dyn Card {
    fn eq(&self, other: &Self) -> bool {
        self.name().eq_ignore_ascii_case(other.name())
    }
}

impl Eq for dyn Card {}

