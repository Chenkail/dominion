//! Defines traits for the various card types

use std::{collections::HashMap, hash::{Hash, Hasher}};
use dyn_clonable::*;

use crate::game::gamedata::*;
use crate::game::player::Player;

/// Card Types
#[derive(PartialEq)]
pub enum CardType {
    // Basic types
    ActionCard,
    TreasureCard,
    VictoryCard,
    CurseCard,
    // Multi-set types
    AttackCard,
    CommandCard,
    DurationCard,
    ReactionCard,
    // Single-set types
    CastleCard,
    DoomCard,
    FateCard,
    GatheringCard,
    HeirloomCard,
    LooterCard,
    NightCard,
    PrizeCard,
    ReserveCard,
    RuinsCard,
    ShelterCard,
    SpiritCard,
    TravellerCard,
    ZombieCard,
}

/// The basic Card trait
///
/// dyn Card implements [Hash] and [Eq] so that Box\<dyn Card\> can be used as keys for a HashMap
#[clonable]
pub trait Card: Clone {
    /// The name on the card (e.g. "Throne Room")
    fn name(&self) -> &str;
    /// The card's types - each type should be title case
    fn types(&self) -> Vec<CardType>;
    /// The card text (this will often be blank, as is the case with all the cards in the base set)
    fn description(&self) -> &str { "" }

    /// How much the card costs to buy, in coins
    fn coin_cost(&self) -> i32;
    /// Potions needed to buy the card
    fn potion_cost(&self) -> i32 { 0 }
    /// Debt in the card cost
    fn debt_cost(&self) -> i32 { 0 }

    // Type check methods
    fn is_action(&self) -> bool { self.types().contains(&CardType::ActionCard) }
    fn is_attack(&self) -> bool { self.types().contains(&CardType::AttackCard) }
    fn is_reaction(&self) -> bool { self.types().contains(&CardType::ReactionCard) }
    fn is_treasure(&self) -> bool { self.types().contains(&CardType::TreasureCard) }
    fn is_victory(&self) -> bool { self.types().contains(&CardType::VictoryCard) }
    fn is_curse(&self) -> bool { self.types().contains(&CardType::CurseCard) }

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
    fn effects_on_play(&self, _player: &mut Player, _supply: &mut Supply, _other_players: &PlayerList) {}
    /// The card's effects when used as a reaction
    fn effects_on_react(&self, _player: &mut Player, _supply: &mut Supply, _other_players: &PlayerList) {}
    /// Effects to trigger when this card is gained
    fn effects_on_gain(&self, _player: &mut Player, _supply: &mut Supply, _other_players: &PlayerList) {}
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
