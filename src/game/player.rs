use std::mem;
use rand::thread_rng;
use rand::seq::SliceRandom;

use crate::game::{cards::base::*, traits::{Action, Card}};

#[derive(Clone, Copy)]
pub struct Resources {
    actions: u32,
    buys: u32,
    temp_coins: u32,
}

impl Resources {
    /// Create a new Resources object
    pub fn new() -> Resources {
        Resources {actions: 1, buys: 1, temp_coins: 0}
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
    pub fn draw_cards(&mut self, cards: u32) {
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

    pub fn play_action(mut self, card: &dyn Action) -> Player {
        self = card.effects(self);
        return self;
    }

    /// Action phase
    pub fn action_phase(&mut self) {
        self.resources.actions = 1;
        self.resources.buys = 1;
        self.resources.temp_coins = 0;

        while self.resources.actions > 0 {
            // Play cards
        }
    }

    /// Buy phase
    pub fn buy_phase(&mut self) {

    }

    /// Cleanup phase at end of turn - discard hand and draw five new cards
    pub fn cleanup(&mut self) {
        for _ in 0..self.hand.len() {
            self.discard.push(self.hand.pop().unwrap());
        }

        self.draw_cards(5);
    }

    /// Print out all cards that the player has
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
