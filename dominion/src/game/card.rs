//! Defines traits for the various card types

use std::fmt::{Display, Formatter, Result};
use std::hash::{Hash, Hasher};
use itertools::Itertools;
use dyn_clonable::*;

use crate::game::gamedata::*;
use crate::game::player::Player;

/// Card Types
#[derive(PartialEq, Eq, Debug)]
pub enum CardType {
    // Basic types
    Action,
    Treasure,
    Victory,
    Curse,
    // Multi-set types
    Attack,
    Command,
    Duration,
    Reaction,
    // Single-set types
    Castle,
    Doom,
    Fate,
    Gathering,
    Heirloom,
    Looter,
    Night,
    Prize,
    Reserve,
    Ruins,
    Shelter,
    Spirit,
    Traveller,
    Zombie,
}

impl Display for CardType {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:?}", *self)
    }
}

/// The basic Card trait
///
/// dyn Card implements [Hash] and [Eq] so that Box\<dyn Card\> can be used as keys for a HashMap
#[clonable]
pub trait Card: Clone {
    /// The name on the card (e.g. "Throne Room")
    fn name(&self) -> &str;
    /// The card's [types](CardType)
    fn types(&self) -> Vec<CardType>;
    /// The card text (this will often be blank, as is the case with all the cards in the base set)
    fn description(&self) -> &str { "" }

    /// How much the card costs to buy, in coins
    fn coin_cost(&self) -> i32;
    /// Debt in the card cost
    fn debt_cost(&self) -> i32 { 0 }
    /// Potions needed to buy the card
    fn potion_cost(&self) -> i32 { 0 }

    // Type check methods
    /// Print out all types a card has, separated by commas
    fn print_types(&self) { println!("{}", self.types().iter().format(", ")) }
    /// Check if this card is an Action
    fn is_action(&self) -> bool { self.types().contains(&CardType::Action) }
    /// Check if this card is an Attack
    fn is_attack(&self) -> bool { self.types().contains(&CardType::Attack) }
    /// Check if this card is a Reaction
    fn is_reaction(&self) -> bool { self.types().contains(&CardType::Reaction) }
    /// Check if this card is a Treasure
    fn is_treasure(&self) -> bool { self.types().contains(&CardType::Treasure) }
    /// Check if this card is a Victory card
    fn is_victory(&self) -> bool { self.types().contains(&CardType::Victory) }
    /// Check if this card is a Curse card
    fn is_curse(&self) -> bool { self.types().contains(&CardType::Curse) }

    /// The number of coins the card is worth (if it is a treasure card)
    fn treasure_value(&self, _player: &Player) -> i32 { 0 }
    /// The number of potions the card is worth (if it is a potion treasure card)
    fn potion_value(&self, _player: &Player) -> i32 { 0 }
    /// The number of points the card is worth (if it is a victory card)
    fn victory_points(&self, _player: &Player) -> i32 { 0 }
    /// The number of points the card is worth (if it is a curse card) - this should be negative
    fn curse_points(&self, _player: &Player) -> i32 { 0 }

    // Effect triggers
    /// The card's effects when played as an action
    fn effects_on_play(&self, _player: &mut Player, _supply: &mut Supply, _other_players: &PlayerSlice) {}
    /// The card's effects when used as a reaction
    fn effects_on_react(&self, _player: &mut Player, _supply: &mut Supply, _other_players: &PlayerSlice) {}
    /// Effects to trigger when this card is gained
    fn effects_on_gain(&self, _player: &mut Player, _supply: &mut Supply, _other_players: &PlayerSlice) {}
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
