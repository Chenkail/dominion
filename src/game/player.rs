use rand::thread_rng;
use rand::seq::SliceRandom;

use super::{cards::base::*, traits::Card};

pub struct Player {
    pub hand: Vec<Box<dyn Card>>,
    pub deck: Vec<Box<dyn Card>>,
    pub discard: Vec<Box<dyn Card>>,
}

impl Player {
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

        Player {hand, deck, discard}
    }

    pub fn actions() {
        
    }

    pub fn buy() {

    }

    pub fn cleanup() {
        
    }
}