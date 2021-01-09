use std::mem;
use rand::thread_rng;
use rand::seq::SliceRandom;

use super::{traits::Card, cards::base::*};

pub struct Player {
    pub hand: Vec<Box<dyn Card>>,
    pub deck: Vec<Box<dyn Card>>,
    pub discard: Vec<Box<dyn Card>>,
}

impl Player {
    /// Create a new player with 3 estates and 7 copper
    pub fn new() -> Player {
        let mut hand: Vec<Box<dyn Card>> = Vec::new();
        let mut deck: Vec<Box<dyn Card>> = Vec::new();
        let discard: Vec<Box<dyn Card>> = Vec::new();
        
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

        for _ in 0..5 {
            hand.push(deck.pop().unwrap());
        }

        Player { hand, deck, discard }
    }

    /// Action phase
    pub fn actions(&self) {
        
    }

    /// Buy phase
    pub fn buy(&self) {

    }

    /// Cleanup phase at end of turn - discard hand and draw five new cards
    pub fn cleanup(&mut self) {
        for _ in 0..self.hand.len() {
            self.discard.push(self.hand.pop().unwrap());
        }

        for _ in 0..5 {
            // If deck is empty, shuffle discard and swap it with the empty deck
            if self.deck.len() == 0 {
                let mut rng = thread_rng();
                self.discard.shuffle(&mut rng);
                mem::swap(&mut self.deck, &mut self.discard);
            }

            self.hand.push(self.deck.pop().unwrap());
        }
    }
}
