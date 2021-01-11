use std::collections::{HashMap, VecDeque};
use crate::game::{player::Player, traits::Card};
use crate::game::cards::all::*;

pub struct Game {
    pub supply: HashMap<Box<dyn Card>, u8>,
    pub trash: VecDeque<Box<dyn Card>>,
}

impl Game {
    /// Create a new game using the recommended set for first time players
    pub fn new(players: u8, cards: Vec<Box<dyn Card>>) -> Game {
        let mut supply: HashMap<Box<dyn Card>, u8> = HashMap::new();
        let trash = VecDeque::new();
        
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
        // TODO: check if card implements victory (is this even possible?)
        for card in cards {
            // let card = card_lookup(&name);
            supply.insert(card, 10);
        }

        Game { supply, trash }
    }
}
