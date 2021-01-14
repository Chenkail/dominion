//! Defines traits for the various card types

use std::hash::{Hash, Hasher};
use dyn_clonable::*;

use crate::game::{gamedata::Game, player::Player};

/// Card Types
#[derive(PartialEq)]
pub enum CardType {
    ActionCard,
    ReactionCard,
    VictoryCard,
    CurseCard,
    AttackCard,
    TreasureCard
}

/// The basic Card trait
///
/// dyn Card implements [Hash] and [Eq] so that Box\<dyn Card\> can be used as keys for a HashMap
#[clonable]
pub trait Card: Clone {
    /// How much the card costs to buy
    fn cost(&self) -> i32;
    /// The name on the card (e.g. "Throne Room")
    fn name(&self) -> &str;
    /// The card's types - each type should be title case
    fn types(&self) -> Vec<CardType>;
    /// The card text (this will often be blank, as is the case with all the cards in the base set)
    fn description(&self) -> &str { "" }

    /// The number of coins the card is worth (if it is a treasure card)
    fn treasure_value(&self, _: &Player) -> i32 { 0 }
    /// The number of points the card is worth (if it is a victory card)
    fn victory_points(&self, _: &Player) -> i32 { 0 }
    /// The number of points the card is worth (if it is a curse card) - this should be negative
    fn curse_points(&self, _: &Player) -> i32 { 0 }
    /// The card's effects when played as an action
    fn action_effects(&self, _: &mut Player, _: &mut Game) {}
    /// The card's effects when used as a reaction
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
