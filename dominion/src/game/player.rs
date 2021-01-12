//! Defines Player object and associated functions

use std::{collections::VecDeque, mem};
use crate::cards::base::*;
use crate::game::{gamedata::Game, traits::*, utils};

/// Struct to keep track of a Player's actions/buys/coins for each turn
struct Resources {
    actions: i32,
    buys: i32,
    coins: i32,
}

impl Resources {
    /// Construct a new [Resources] struct
    pub fn new() -> Resources {
        Resources {actions: 1, buys: 1, coins: 0}
    }
}

/// Struct representing a player
pub struct Player { 
    pub hand: VecDeque<Box<dyn Card>>,
    pub deck: VecDeque<Box<dyn Card>>,
    pub discard: VecDeque<Box<dyn Card>>,
    resources: Resources,
}

impl Player {
    /// Construct a new [Player] with 3 estates and 7 copper
    pub fn new() -> Player {
        let mut hand: VecDeque<Box<dyn Card>> = VecDeque::new();
        let mut deck: VecDeque<Box<dyn Card>> = VecDeque::new();
        let discard: VecDeque<Box<dyn Card>> = VecDeque::new();
        let resources = Resources::new();
        
        for _ in 0..7 {
            let copper = Box::new(Copper);
            deck.push_back(copper);
        }
        
        for _ in 0..3 {
            let estate = Box::new(Estate);
            deck.push_back(estate);
        }

        utils::shuffle(&mut deck);

        // Initial hand of 5 cards
        for _ in 0..5 {
            hand.push_back(deck.pop_front().unwrap());
        }

        Player { hand, deck, discard, resources }
    }

    /// Draw x cards for the player
    pub fn draw_cards(&mut self, cards: i32) {
        for _ in 0..cards {
            // If deck is empty, shuffle discard and swap it with the empty deck
            if self.deck.len() == 0 {
                utils::shuffle(&mut self.discard);
                mem::swap(&mut self.deck, &mut self.discard);
            }

            self.hand.push_back(self.deck.pop_front().unwrap());
        }
    }

    /// Discard cards from hand given an array of indexes of said cards
    pub fn discard_given_indexes(&mut self, mut indexes: Vec<usize>) {        
        indexes.sort();
        indexes.reverse();
        for i in indexes {
            self.discard.push_back(self.hand.remove(i).unwrap());
        }
    }

    /// Trash cards from hand given an array of indexes of said cards
    pub fn trash_given_indexes(&mut self, mut indexes: Vec<usize>, game: &mut Game) {
        indexes.sort();
        indexes.reverse();
        for i in indexes {
            game.trash.push_back(self.hand.remove(i).unwrap());
        }
    }

    /// Add extra actions for the player for this turn
    pub fn add_actions(&mut self, actions: i32) {
        self.resources.actions += actions;
    }

    /// Add extra buys for the player for this turn
    pub fn add_buys(&mut self, buys: i32) {
        self.resources.buys += buys;
    }

    /// Add extra coins for the player for this turn
    pub fn add_coins(&mut self, coins: i32) {
        self.resources.coins += coins;
    }

    /// Play an action [card](Card) from the player's hand
    ///
    /// This is the function to call when a player plays a card directly
    pub fn play_action_from_hand(&mut self, card: &dyn Action, game: &mut Game) {
        self.resources.actions -= 1;
        self.action_effects(card, game);
    }

    /// Give the player the effects of an action card as if they had played it
    ///
    /// Does not subtract actions from the player's total. Should only be called
    /// in the effects() function of other cards (e.g. Throne Room)
    pub fn action_effects(&mut self, card: &dyn Action, game: &mut Game) {
        card.effects(self, game);
    }

    /// Action phase
    pub fn action_phase(&mut self, game: &mut Game) {
        // Reset resources
        self.resources.actions = 1;
        self.resources.buys = 1;
        self.resources.coins = 0;

        while self.resources.actions > 0 {
            // Play cards
        }
    }

    /// Buy a card
    pub fn buy_card(&mut self, card: Box<dyn Card>, game: &mut Game) {
        // TODO: check if supply pile is empty
        *game.supply.get_mut(&card).unwrap() -= 1;

        self.resources.coins -= card.cost();
        self.discard.push_back(card);
        
    }

    /// Buy phase
    pub fn buy_phase(&mut self, game: &mut Game) {
        // TODO: Count money


        while self.resources.buys > 0 {
            // Buy cards

        }
    }

    pub fn count_money_in_hand(&self) -> i32 {
        // Add coins from treasures in hand to total
        for card in &self.hand {
            // TODO: Check if is treasure, then add value
            
        }
        
        -1
    }

    /// Cleanup phase at end of turn - discard hand and draw five new cards
    pub fn cleanup(&mut self) {
        for _ in 0..self.hand.len() {
            self.discard.push_back(self.hand.pop_front().unwrap());
        }

        self.draw_cards(5);
    }

    /// Take a turn
    pub fn turn(&mut self, game: &mut Game) {
        self.action_phase(game);
        self.buy_phase(game);
        self.cleanup();
    }

    /// Count up a player's victory points
    pub fn victory_points(&self) -> i32 {
        let mut points = 0;
        for card in &self.hand {
            
        }
        for card in &self.deck {
            
        }
        for card in &self.discard {
        
        }
        return points;
    }

    /// Print out resources along with number of cards in hand/deck/discard/total
    pub fn print_state(&self) {
        let hand_count = self.hand.len();
        let deck_count = self.deck.len();
        let discard_count = self.discard.len();
        let indent = "    ";
        
        println!("Cards in");
        print!("{}", indent);
        println!("hand: {}", hand_count);
        print!("{}", indent);
        println!("deck: {}", deck_count);
        print!("{}", indent);
        println!("discard: {}", discard_count);
        println!("Total cards: {}", hand_count + deck_count + discard_count);
        
        println!("Actions: {}", self.resources.actions);
        println!("Buys: {}", self.resources.buys);
        println!("Coins: {}", self.resources.coins);
    }

    /// Print out all cards that the player has, in order, and where they are
    pub fn print_cards(&self) {
        let indent = "    ";

        println!("Hand:");
        print!("{}", indent);
        for card in &self.hand {
            print!("{}, ", card.name());
        }
        println!();

        println!("Deck:");
        print!("{}", indent);
        for card in &self.deck {
            print!("{}, ", card.name());
        }
        println!();
        
        println!("Discard:");
        print!("{}", indent);
        for card in &self.discard {
            print!("{}, ", card.name());
        }
        println!();
    }
}
