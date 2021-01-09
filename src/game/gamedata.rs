use std::{collections::HashMap, vec};
use super::{cards::{self, base::*}, traits::Card};
pub struct Game {
    pub supply: HashMap<&'static str, u8>,
}

impl Game {
    /// Create a new game using the recommended set for first time players
    pub fn default(players: u8) -> Game {
        let mut supply = HashMap::new();
        let kingdom_cards = vec!["Cellar", "Market", "Merchant", "Militia", "Mine", 
                                    "Moat", "Remodel", "Smithy", "Village", "Workshop"];
        let kingdom_count;
        match players {
            2 => {
                supply.insert("Estate", 8);
                supply.insert("Duchy", 8);
                supply.insert("Province", 8);
                supply.insert("Curse", 10);
                kingdom_count = 8;
            }
            
            3 => {
                supply.insert("Estate", 12);
                supply.insert("Duchy", 12);
                supply.insert("Province", 12);
                supply.insert("Curse", 20);
                kingdom_count = 12;
            }

            4 => {
                supply.insert("Estate", 12);
                supply.insert("Duchy", 12);
                supply.insert("Province", 12);
                supply.insert("Curse", 30);
                kingdom_count = 12;
            }

            _ => panic!("Invalid player count!")
        }
        
        for card in kingdom_cards {
            supply.insert(card, kingdom_count);
        }

        Game { supply }
    }
}
