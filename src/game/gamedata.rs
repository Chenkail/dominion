//! Defines the Game struct and its behavior

use serde::{Serialize, Deserialize};

use crate::game::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Game {
    pub players: PlayerList,
    pub supply: Supply,
    pub trash: CardDeck,
    pub extras: Supply,
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
        let extras: Supply = HashMap::new();

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

        Game { players: player_vec, supply, trash , extras }
    }

    /// Prints out all the cards in the supply and their remaining quantities
    pub fn print_supply(&mut self) {
        for (card, count) in &self.supply {
            println!("{}: {} cards", card, count);
        }
    }

    /// checks game state, used for victory condition.
    /// returns true if the province cards are exhausted OR when 3 stacks in the supply are exhausted.
    pub fn victory_met(&mut self) -> bool {
        let province: Box<dyn Card> = Box::new(Province);
        if *self.supply.get(&province).unwrap() as u32 == 0 {
            return true;
        }

        if self.supply
            .values()
            .filter(|e| **e == 0)
            .count() == 3 {
            return true;
        };

        false
    }

    /// returns vector of cards available under a certain cost
    /// hopefully we can combine this and related methods into one generic one
    fn return_avail_cards_ucost(supply: Supply, cost: usize) -> Vec<Box<dyn Card>> {
        //TODO: rewrite to not use collect and to use filter() with the lambda passed in
        return supply.keys()
            .filter(|a| *supply.get(*a).unwrap() > 0)
            .filter(|a| a.coin_cost() < cost)
            .cloned()
            .collect();
    }

    /// returns vector of cards available above a certain cost
    /// hopefully we can combine this and related methods into one generic one
    fn return_avail_cards_acost(supply: Supply, cost: usize) -> Vec<Box<dyn Card>> {
        //TODO: rewrite to not use collect and to use filter() with the lambda passed in
        return supply.keys()
            .filter(|a| *supply.get(*a).unwrap() > 0)
            .filter(|a| a.coin_cost() > cost)
            .cloned()
            .collect();
    }

     /// returns vector of cards available of a certain type
    /// hopefully we can combine this and related methods into one generic one
    fn return_avail_cards_type(supply: Supply, t: CardType) -> Vec<Box<dyn Card>> {
        //TODO: rewrite to not use collect and to use filter() with the lambda passed in
        return supply.keys()
            .filter(|a| *supply.get(*a).unwrap() > 0)
            .filter(|a| a.types().contains(&t))
            .cloned()
            .collect();
    }
}
