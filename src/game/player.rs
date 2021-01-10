use std::mem;
use rand::thread_rng;
use rand::seq::SliceRandom;

use crate::game::{cards::base::*, traits::{Action, Card}};

#[derive(Clone, Copy)]
pub struct Resources {
    actions: i32,
    buys: i32,
    coins: i32,
}

impl Resources {
    /// Create a new Resources object
    pub fn new() -> Resources {
        Resources {actions: 1, buys: 1, coins: 0}
    }
}

pub struct Player {
    pub hand: Vec<Box<dyn Card>>,
    pub deck: Vec<Box<dyn Card>>,
    pub discard: Vec<Box<dyn Card>>,
    resources: Resources,
}

impl Player {
    /// Create a new player with 3 estates and 7 copper
    pub fn new() -> Player {
        let mut hand: Vec<Box<dyn Card>> = Vec::new();
        let mut deck: Vec<Box<dyn Card>> = Vec::new();
        let discard: Vec<Box<dyn Card>> = Vec::new();
        let resources = Resources::new();
        
        for _ in 0..7 {
            let copper = Box::new(Copper);
            deck.push(copper);
        }
        
        for _ in 0..3 {
            let estate = Box::new(Estate);
            deck.push(estate);
        }

        let mut rng = thread_rng();
        deck.shuffle(&mut rng);

        // Initial hand of 5 cards
        for _ in 0..5 {
            hand.push(deck.pop().unwrap());
        }

        Player { hand, deck, discard, resources }
    }

    /// Draws x cards for the player
    pub fn draw_cards(&mut self, cards: i32) {
        for _ in 0..cards {
            // If deck is empty, shuffle discard and swap it with the empty deck
            if self.deck.len() == 0 {
                let mut rng = thread_rng();
                self.discard.shuffle(&mut rng);
                mem::swap(&mut self.deck, &mut self.discard);
            }

            self.hand.push(self.deck.pop().unwrap());
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

    /// Plays a non-attack action card
    pub fn play_action(mut self, card: &dyn Action) -> Player {
        self.resources.actions -= 1;
        self = card.effects(self);
        return self;
    }

    /// Action phase
    pub fn action_phase(&mut self) {
        // Reset resources
        self.resources.actions = 1;
        self.resources.buys = 1;
        self.resources.coins = 0;

        while self.resources.actions > 0 {
            // Play cards
        }
    }

    /// Buy a card
    pub fn buy_card(&mut self, card: Box<dyn Card>) {
        self.resources.coins -= card.cost();
        self.discard.push(card);
    }

    /// Buy phase
    pub fn buy_phase(&mut self) {
        // Add coins from treasures in hand to total
        for card in &self.hand {
            // TODO: Check if is treasure, then add value

        }

        while self.resources.buys > 0 {
            // Buy cards

        }
    }

    /// Cleanup phase at end of turn - discard hand and draw five new cards
    pub fn cleanup(&mut self) {
        for _ in 0..self.hand.len() {
            self.discard.push(self.hand.pop().unwrap());
        }

        self.draw_cards(5);
    }

    /// Take a turn
    pub fn turn(&mut self) {
        self.action_phase();
        self.buy_phase();
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

    /// Print out all cards that the player has, in order, and where they are
    pub fn print_cards(&self) {
        println!("Hand:");
        for card in &self.hand {
            print!("{}, ", card.name());
        }
        println!();

        println!("Deck:");
        for card in &self.deck {
            print!("{}, ", card.name());
        }
        println!();
        
        println!("Discard:");
        for card in &self.discard {
            print!("{}, ", card.name());
        }
        println!();
    }
}
