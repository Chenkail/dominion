use std::collections::HashMap;
use crate::game::{player::Player, utils::card_lookup, cards::base::*, traits::Card};

pub struct Game {
    pub supply: HashMap<&'static str, u8>,
}

impl Game {
    /// Create a new game using the recommended set for first time players
    pub fn default(players: u8) -> Game {
        let mut supply = HashMap::new();
        let kingdom_cards = vec!["Cellar", "Market", "Merchant", "Militia", "Mine", 
                                            "Moat", "Remodel", "Smithy", "Village", "Workshop"];
        let victory_count;
        match players {
            2 => {
                supply.insert("Curse", 10);
                victory_count = 8;
            }
            
            3 => {
                supply.insert("Curse", 20);
                victory_count = 12;
            }

            4 => {
                supply.insert("Curse", 30);
                victory_count = 12;
            }

            _ => panic!("Invalid player count!")
        }

        supply.insert("Estate", victory_count);
        supply.insert("Duchy", victory_count);
        supply.insert("Province", victory_count);
        
        // If card is victory card, count matches other victory cards
        // Otherwise use 10 copies
        // TODO: check if card implements victory (is this even possible?)
        for name in kingdom_cards {
            // let card = card_lookup(&name);
            supply.insert(name, 10);
        }

        Game { supply }
    }
}
