//! Defines Player object and associated functions

use std::{collections::{HashMap, VecDeque}, mem};
use crate::cards::base::*;
use crate::game::{card::*, card::CardType::*, utils};
use crate::error::DominionError;
use DominionError::*;
use dominion_macros::card_vec;

/// Struct to keep track of a Player's actions/buys/coins for each turn
#[derive(Default)]
pub struct Resources {
    pub actions: i32,
    pub buys: i32,
    pub coins_in_hand: i32,
    pub temp_coins: i32,
    pub potions_in_hand: i32,
    pub temp_potions: i32,
    pub debt: i32,
}

/// Struct representing a player
pub struct Player { 
    pub hand: VecDeque<Box<dyn Card>>,
    pub deck: VecDeque<Box<dyn Card>>,
    pub discard: VecDeque<Box<dyn Card>>,
    pub in_play: VecDeque<Box<dyn Card>>,
    pub resources: Resources,
}

impl Default for Player {
    /// Constructs a new [Player] with 3 estates and 7 copper
    fn default() -> Player {
        let deck = card_vec![Copper, Copper, Copper, Copper, Copper, Copper, Copper, Estate, Estate, Estate];
        Player::new(deck)
    }
}

impl Player {
    pub fn new (cards: Vec<Box<dyn Card>>) -> Player {
        let mut hand: VecDeque<Box<dyn Card>> = VecDeque::new();
        let mut deck: VecDeque<Box<dyn Card>> = VecDeque::from(cards);
        let discard: VecDeque<Box<dyn Card>> = VecDeque::new();
        let in_play: VecDeque<Box<dyn Card>> = VecDeque::new();
        let resources = Resources::default();

        utils::shuffle(&mut deck);

        // Initial hand of 5 cards
        for _ in 0..5 {
            hand.push_back(deck.pop_front().unwrap());
        }

        Player { hand, deck, discard, in_play, resources }
    }

    /// Gets an iterator with references to all cards in the player's hand, deck, and discard
    pub fn card_iter(&self) -> impl Iterator<Item = &Box<dyn Card>> {
        return self.hand.iter()
                .chain(self.deck.iter())
                .chain(self.discard.iter())
                .chain(self.in_play.iter());
    }

    /// Draws x cards for the player
    pub fn draw_cards(&mut self, cards: i32) {
        for _ in 0..cards {
            // If deck is empty, shuffle discard and swap it with the empty deck
            if self.deck.is_empty() {
                // If discard is also empty, there is nothing to draw
                if self.discard.is_empty() {
                    return;
                }

                utils::shuffle(&mut self.discard);
                mem::swap(&mut self.deck, &mut self.discard);
            }

            self.hand.push_back(self.deck.pop_front().unwrap());
        }
    }

    /// Discards cards from hand given an array of indexes of said cards
    pub fn discard_given_indexes(&mut self, mut indexes: Vec<usize>) {
        indexes.sort_unstable();
        indexes.reverse();
        for i in indexes {
            self.discard.push_back(self.hand.remove(i).unwrap());
        }
    }

    /// Trashes cards from hand given an array of indexes of said cards
    pub fn trash_given_indexes(&mut self, mut indexes: Vec<usize>, trash: &mut VecDeque<Box<dyn Card>>) {
        indexes.sort_unstable();
        indexes.reverse();
        for i in indexes {
            trash.push_back(self.hand.remove(i).unwrap());
        }
    }

    /// Gives the player extra actions for this turn
    pub fn add_actions(&mut self, actions: i32) {
        self.resources.actions += actions;
    }

    /// Gives the player extra buys for this turn
    pub fn add_buys(&mut self, buys: i32) {
        self.resources.buys += buys;
    }

    /// Gives the player extra coins for this turn
    pub fn add_coins(&mut self, coins: i32) {
        self.resources.temp_coins += coins;
    }

    /// Plays an action [card](Card) from the player's hand
    ///
    /// This is the function to call when a player plays a card directly
    pub fn play_action_from_hand(&mut self, index: usize, supply: &mut HashMap<Box<dyn Card>, u8>, other_players: &mut Vec<Player>) -> Result<(), DominionError> {
        // Remove card from hand
        let card = self.hand.get(index).unwrap();
        if card.is_action() {
            let card = self.hand.remove(index).unwrap();
            self.in_play.push_back(card.clone());

            self.resources.actions -= 1;
            self.action_effects(&*card, supply, other_players);

            Ok(())
        } else {
            Err(CardTypeMisMatch { expected: "Action".to_string() })
        }
    }

    /// Gives the player the effects of an action card as if they had played it
    ///
    /// Does not subtract actions from the player's total. Should only be called
    /// in the effects() function of other cards (e.g. Throne Room)
    pub fn action_effects(&mut self, card: &dyn Card, supply: &mut HashMap<Box<dyn Card>, u8>, other_players: &mut Vec<Player>) {
        card.action_effects(self, supply, other_players);
    }

    /// Action phase
    pub fn action_phase(&mut self, supply: &mut HashMap<Box<dyn Card>, u8>, other_players: &Vec<Player>) {
        // Reset resources
        self.resources.actions = 1;
        self.resources.buys = 1;
        self.resources.temp_coins = 0;

        //TODO (much later): Duration cards

        while self.resources.actions > 0 {
            // TODO: Figure out how to allow player to declare that they are done playing actions
        }
    }

    /// Buy a card
    pub fn buy_card(&mut self, card: Box<dyn Card>, supply: &mut HashMap<Box<dyn Card>, u8>) {
        // TODO: check if supply pile is empty
        *supply.get_mut(&card).unwrap() -= 1;

        self.resources.temp_coins -= card.coin_cost();
        self.discard.push_back(card);
        
    }

    /// Buy phase
    pub fn buy_phase(&mut self, supply: &mut HashMap<Box<dyn Card>, u8>) {
        let mut coins_remaining = self.resources.coins_in_hand + self.resources.temp_coins;
        let mut potions_remaining = self.resources.coins_in_hand + self.resources.temp_coins;

        if self.resources.debt > 0 {
            // Can't buy
            //TODO: pay off debt
            return;
        }

        while self.resources.buys > 0 {
            // Buy cards
            // TODO: Figure out how to allow player to declare that they are done buying cards
            self.buy_card(Box::new(Copper), supply)
        }
    }

    pub fn play_treasure(&mut self, index: usize, supply: &mut HashMap<Box<dyn Card>, u8>, other_players: &mut Vec<Player>) -> Result<(), DominionError> {
        // Remove card from hand
        let card = self.hand.get(index).unwrap();
        if card.is_treasure() {
            self.resources.coins_in_hand += card.treasure_value(self);

            Ok(())
        } else {
            Err(CardTypeMisMatch { expected: "TreasureCard".to_string() })
        }
    }

    pub fn play_all_treasures(&mut self, index: usize, supply: &mut HashMap<Box<dyn Card>, u8>, other_players: &mut Vec<Player>) -> Result<(), DominionError> {
        for i in 0..self.hand.len() {
            let card = self.hand.get(index).unwrap();
            if card.is_treasure() {
                self.play_treasure(i, supply, other_players)?;
            }
        }

        Ok(())
    }

    /// Returns the total value of the treasure cards in the player's hand
    pub fn update_coins_in_hand(&mut self) {
        // Add coins from treasures in hand to total
        let mut total = 0;
        for card in &self.hand {
            total += card.treasure_value(self);
        }

        self.resources.coins_in_hand = total;
    }

    /// Returns the total value of the potion cards in the player's hand
    pub fn update_potions_in_hand(&mut self) {
        // Add coins from treasures in hand to total
        let mut total = 0;
        for card in &self.hand {
            total += card.potion_value(self);
        }

        self.resources.potions_in_hand = total;
    }

    /// Cleanup phase at end of turn - discard hand and draw five new cards
    pub fn cleanup(&mut self) {
        for _ in 0..self.hand.len() {
            self.discard.push_back(self.hand.pop_front().unwrap());
        }

        self.draw_cards(5);
    }

    /// Take a turn
    pub fn turn(&mut self, supply: &mut HashMap<Box<dyn Card>, u8>, other_players: &mut Vec<Player>) {
        self.action_phase(supply, other_players);
        self.buy_phase(supply);
        self.cleanup();
    }

    /// Count up a player's victory points
    pub fn victory_points(&self) -> i32 {
        let mut points = 0;
        let iter = self.card_iter();
        for card in iter {
            points += card.victory_points(self);
        }
        points
    }

    /// Prints out resources along with number of cards in hand/deck/discard/total
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
        println!("Coins: {}", self.resources.temp_coins);
    }

    /// Prints out all cards that the player has, in order, and where they are
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
