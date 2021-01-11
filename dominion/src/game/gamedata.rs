use std::collections::{HashMap, VecDeque};
use crate::game::{player::Player, utils::card_lookup, cards::base::*, traits::Card};
use crate::game::cards::{base::*,
                            dominion::*};

pub struct Game {
    pub supply: HashMap<Box<dyn Card>, u8>,
    pub trash: VecDeque<Box<dyn Card>>,
}

impl Game {
    /// Create a new game using the recommended set for first time players
    pub fn new(players: u8, cards: Vec<Box<dyn Card>>) -> Game {
        let mut supply: HashMap<Box<dyn Card>, u8> = HashMap::new();
        let trash = VecDeque::new();
        
        let victory_card_count;
        match players {
            2 => {
                // supply.insert("Curse", 10);
                victory_card_count = 8;
            }
            
            3 => {
                // supply.insert("Curse", 20);
                victory_card_count = 12;
            }

            4 => {
                // supply.insert("Curse", 30);
                victory_card_count = 12;
            }

            _ => panic!("Invalid player count!")
        }

        // supply.insert("Estate", victory_card_count);
        // supply.insert("Duchy", victory_card_count);
        // supply.insert("Province", victory_card_count);
        
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
