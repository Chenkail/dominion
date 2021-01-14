//! Defines the Game struct and its behavior

use std::collections::{HashMap, VecDeque};
use crate::cards::all::*;
use crate::game::{player::Player, card::Card};

// defines the Types Enum for use in Cards
// may be moved somewhere else later? perhaps a types.rs file
pub enum CardType {
    ActionCard,
    ReactionCard,
    VictoryCard,
    CurseCard,
    AttackCard,
    TreasureCard
}

pub struct Game {
    pub players: Vec<Player>,
    pub supply: HashMap<Box<dyn Card>, u8>,
    pub trash: VecDeque<Box<dyn Card>>,
}

impl Game {
    /// Create a new [Game] given a list of [Cards](Card) for the supply
    pub fn new(players: u8, cards: Vec<Box<dyn Card>>) -> Game {
        let mut player_vec: Vec<Player> = Vec::with_capacity(players as usize);
        let mut supply: HashMap<Box<dyn Card>, u8> = HashMap::new();
        let trash = VecDeque::new();

        for _ in 0..players {
            player_vec.push(Player::new())
        }
        
        let (victory_card_count, province_count, curse_count) = match players {
            2 => (8, 8, 10),
            3 => (12, 12, 20),
            4 => (12, 12, 30),
            5 => (12, 15, 40),
            6 => (12, 18, 50),
            _ => panic!("Invalid player count!")
        };

        supply.insert(Box::new(Estate), victory_card_count);
        supply.insert(Box::new(Duchy), victory_card_count);
        supply.insert(Box::new(Province), province_count);
        supply.insert(Box::new(Curse), curse_count);
        
        // If card is victory card, count matches other victory cards
        // Otherwise use 10 copies
        for card in cards {
            let count = if card.types().contains(&"Victory") {
                victory_card_count
            } else {
                10
            };
            supply.insert(card, count);
        }

        Game { players: player_vec, supply, trash }
    }

    /// Prints out all the cards in the supply and their remaining quantities
    pub fn print_supply() {

    }
}
