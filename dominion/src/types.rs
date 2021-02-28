//! Library type definitions.

use std::{collections::{HashMap, VecDeque}, fmt};
use std::fmt::{Display, Formatter};

use serde::{Serialize, Deserialize};

use crate::game::Player;
use crate::game::Card;

pub type PlayerList = Vec<Player>;
pub type PlayerSlice = [Player];
pub type CardList = Vec<Box<dyn Card>>;
pub type CardDeck = VecDeque<Box<dyn Card>>;
pub type Supply = HashMap<Box<dyn Card>, u8>;

/// Card Types
#[derive(PartialEq, Eq, Debug)]
#[derive(Serialize, Deserialize)]
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
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}", *self)
    }
}

/// Which players an attack can target
#[derive(PartialEq, Eq, Debug)]
#[derive(Serialize, Deserialize)]
pub enum AttackTargetType {
    PlayerToLeft,
    EveryoneElse,
    // PlayerOfChoice,
}

/// Triggers for Reaction cards
#[derive(PartialEq, Eq, Debug)]
#[derive(Serialize, Deserialize)]
pub enum ReactionTrigger {
    // Most common reaction trigger - another player plays an attack card
    OtherPlayerPlaysAttack,
    // Hovel
    BuyAVictoryCard,
    // Black Cat
    OtherPlayerGainsVictoryCard,
    // Fool's Gold
    OtherPlayerGainsProvince,
    // Faithful Hound, Tunnel, Village Green
    DiscardedNotDuringCleanup,
    // Sheepdog, Sleigh, Trader, Watchtower
    YouGainACard,
    // Market Square
    CardIsTrashed,
    // Patron
    YouRevealThis,
    // Falconer
    FalconerReaction,
}
