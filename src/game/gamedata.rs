//! Defines the Game struct and its behavior

use std::collections::{HashMap, VecDeque};
use serde::{Serialize, Deserialize};

use dominion_macros::*;
use crate::cards::all::*;
use crate::game::{player::Player, card::Card};

pub type PlayerList = Vec<Player>;
pub type PlayerSlice = [Player];
pub type CardList = Vec<Box<dyn Card>>;
pub type CardDeck = VecDeque<Box<dyn Card>>;
pub type Supply = HashMap<Box<dyn Card>, u8>;
#[derive(Debug, Serialize, Deserialize)]
pub struct Game {
    pub players: PlayerList,
    pub supply: Supply,
    pub trash: CardDeck,
}

impl Default for Game {
    /// Creates a two-player game with the recommended first game set
    fn default() -> Self {
        let kingdom_cards: CardList = card_vec![Cellar, Market, Merchant, Militia, Mine, Moat, Remodel, Smithy, Village, Workshop];
        Game::new(2, kingdom_cards)
    }
}

impl Game {
    /// Create a new [Game] given a list of [Cards](Card) for the supply
    pub fn new(player_count: usize, cards: CardList) -> Game {
        let mut player_vec: PlayerList = Vec::with_capacity(player_count);
        let mut supply: Supply = HashMap::new();
        let trash: CardDeck = VecDeque::new();

        for i in 0..player_count {
            player_vec.push(Player::new_with_default_deck(i))
        }

        let (victory_card_count, province_count, curse_count) = match player_count {
            2 => (8, 8, 10),
            3 => (12, 12, 20),
            4 => (12, 12, 30),
            5 => (12, 15, 40),
            6 => (12, 18, 50),
            _ => panic!("Invalid player count!")
        };

        supply.insert(Box::new(Copper), 40);
        supply.insert(Box::new(Silver), 40);
        supply.insert(Box::new(Gold), 40);
        supply.insert(Box::new(Estate), victory_card_count);
        supply.insert(Box::new(Duchy), victory_card_count);
        supply.insert(Box::new(Province), province_count);
        supply.insert(Box::new(BasicCurse), curse_count);
        
        // If card is victory card, count matches other victory cards
        // Otherwise use 10 copies
        for card in cards {
            let count = if card.is_victory() {
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
        // TODO
    }
}
