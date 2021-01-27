//! Defines Player object and associated functions

use std::{collections::VecDeque, mem};
use serde::{Serialize, Deserialize};

use crate::game::prelude::*;
use crate::error::DominionError;
use DominionError::*;
use dominion_macros::card_vec;

/// Struct to keep track of a Player's actions/buys/coins for each turn
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Resources {
    pub actions: usize,
    pub buys: usize,
    pub coins_in_hand: usize,
    pub temp_coins: usize,
    pub coins_remaining: usize,
    pub potions_in_hand: usize,
    pub temp_potions: usize,
    pub potions_remaining: usize,
    pub debt: usize,
}

/// Struct representing a player
#[derive(Debug, Serialize, Deserialize)]
pub struct Player {
    pub id: usize,
    pub hand: CardDeck,
    pub deck: CardDeck,
    pub discard: CardDeck,
    pub actions_in_play: CardDeck,
    pub treasures_in_play: CardDeck,
    pub resources: Resources,
}

impl Player {
    /// Constructs a new Player with the default deck (3 estates and 7 copper)
    pub fn new_with_default_deck(id: usize) -> Player {
        let deck = card_vec![Copper, Copper, Copper, Copper, Copper, Copper, Copper, Estate, Estate, Estate];
        Player::new(id, deck)
    }

    /// Constructs a new Player with a given deck
    pub fn new (id: usize, cards: CardList) -> Player {
        let mut hand: CardDeck = VecDeque::new();
        let mut deck: CardDeck = VecDeque::from(cards);
        let discard: CardDeck = VecDeque::new();
        let actions_in_play: CardDeck = VecDeque::new();
        let treasures_in_play: CardDeck = VecDeque::new();
        let resources = Resources::default();

        utils::shuffle(&mut deck);

        // Initial hand of 5 cards
        for _ in 0..5 {
            hand.push_back(deck.pop_front().unwrap());
        }

        Player { id, hand, deck, discard, actions_in_play, treasures_in_play, resources }
    }

    /// Gets an iterator with references to all cards in the player's hand, deck, and discard
    pub fn card_iter(&self) -> impl Iterator<Item = &Box<dyn Card>> {
        return self.hand.iter()
                .chain(self.deck.iter())
                .chain(self.discard.iter())
                .chain(self.actions_in_play.iter());
    }

    /// Draws x cards for the player
    pub fn draw_cards(&mut self, cards: usize) {
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
    /// indexes should be valid: it is up to the callee function to make sure.
    pub fn discard_given_indexes(&mut self, mut indexes: Vec<usize>) {        

        indexes.sort_unstable();
        indexes.reverse();
        for i in indexes {
            //if hand is empty, return from function
            if self.hand.is_empty() {
                return;
            }
            self.discard.push_back(self.hand.remove(i).unwrap());
            
            
        }
    }

    /// Trashes cards from hand given an array of indexes of said cards
    /// indexes should be valid: it is up to the callee function to make sure.
    pub fn trash_given_indexes(&mut self, mut indexes: Vec<usize>, trash: &mut CardDeck) {
        indexes.sort_unstable();
        indexes.reverse();
        for i in indexes {
            //if hand is empty, return from function
            if self.hand.is_empty() {
                return;
            }
            trash.push_back(self.hand.remove(i).unwrap());
            
        }
    }

    /// Gives the player extra actions for this turn
    pub fn add_actions(&mut self, actions: usize) {
        self.resources.actions += actions;
    }

    /// Gives the player extra buys for this turn
    pub fn add_buys(&mut self, buys: usize) {
        self.resources.buys += buys;
    }

    /// Gives the player extra coins for this turn
    pub fn add_coins(&mut self, coins: usize) {
        self.resources.temp_coins += coins;
    }

    /// Plays an action [card](Card) from the player's hand
    ///
    /// This is the function to call when a player plays a card directly
    pub fn play_action_from_hand(&mut self, index: usize, supply: &mut Supply, other_players: &mut PlayerSlice, callbacks: &Callbacks) -> Result<(), DominionError> {
        // Remove card from hand
        let card = self.hand.get(index).unwrap();
        if card.is_action() {
            let card = self.hand.remove(index).unwrap();
            self.actions_in_play.push_back(card.clone());

            self.resources.actions -= 1;
            self.action_effects(&*card, supply, other_players, callbacks);

            Ok(())
        } else {
            Err(CardTypeMisMatch { expected: "Action".to_string() })
        }
    }

    /// Gives the player the effects of an action card as if they had played it
    ///
    /// Does not subtract actions from the player's total. Should only be called
    /// in the effects() function of other cards (e.g. Throne Room)
    pub fn action_effects(&mut self, card: &dyn Card, supply: &mut Supply, other_players: &mut PlayerSlice, callbacks: &Callbacks) {
        card.effects_on_play(self, supply, other_players, callbacks);
    }

    /// Action phase
    pub fn action_phase(&mut self, supply: &mut Supply, other_players: &mut PlayerSlice, callbacks: &Callbacks) {
        // Reset resources
        self.resources.actions = 1;
        self.resources.buys = 1;
        self.resources.temp_coins = 0;

        //TODO (much later): Duration cards

        let mut more = true;
        while (self.resources.actions > 0) & more {
            // TODO: Figure out how to allow player to declare that they are done playing actions
            more = (callbacks.prompt_player_done)();
            if more {
                for i in 0..self.hand.len() {
                    let card = self.hand.get(i).unwrap();
                    if card.is_action() {
                        self.resources.actions -= 1;
                        self.play_action_from_hand(i, supply, other_players, callbacks).unwrap();
                        break;
                    }
                }
            }
        }
    }

    /// Gain a copy of a card to the discard pile
    pub fn gain(&mut self, card: Box<dyn Card>, supply: &mut Supply, other_players: &PlayerSlice, callbacks: &Callbacks) {
        // TODO: check if supply pile is empty
        *supply.get_mut(&card).unwrap() -= 1;
        card.effects_on_gain(self, supply, other_players, callbacks);
        self.discard.push_back(card);
    }

    /// Gain a copy of a card to hand
    pub fn gain_to_hand(&mut self, card: Box<dyn Card>, supply: &mut Supply, other_players: &PlayerSlice, callbacks: &Callbacks) {
        // TODO: check if supply pile is empty
        *supply.get_mut(&card).unwrap() -= 1;
        card.effects_on_gain(self, supply, other_players, callbacks);
        self.hand.push_back(card);
    }

    /// Buy a card
    pub fn buy_card(&mut self, card: Box<dyn Card>, supply: &mut Supply, other_players: &PlayerSlice, callbacks: &Callbacks) {
        card.effects_on_gain(self, supply, other_players, callbacks);
        self.resources.temp_coins -= card.coin_cost();
        self.gain(card, supply, other_players, callbacks);

        self.resources.buys -= 1;
    }

    /// Buy phase
    pub fn buy_phase(&mut self, supply: &mut Supply, other_players: &PlayerSlice, callbacks: &Callbacks) {
        // TODO: prompt user to play treasures

        self.resources.coins_remaining = self.resources.coins_in_hand + self.resources.temp_coins;
        self.resources.potions_remaining = self.resources.potions_in_hand + self.resources.temp_potions;

        if self.resources.debt > 0 {
            // Can't buy
            // TODO: prompt player to acknowledge paying off debt
            // If outstanding debt exists after paying, player cannot buy
            if self.resources.coins_remaining < self.resources.debt {
                self.resources.debt -= self.resources.coins_remaining;
                return;
            } else {
                self.resources.coins_remaining -= self.resources.debt;
                self.resources.debt = 0;
            }
        }

        let mut more = true;
        while (self.resources.buys > 0) & more {
            // Buy cards
            // TODO: Figure out how to allow player to choose card they want

            self.buy_card(Box::new(Copper), supply, other_players, callbacks);

            more = (callbacks.prompt_player_done)();
        }
    }

    pub fn play_treasure(&mut self, index: usize, supply: &mut Supply, other_players: &mut PlayerSlice, callbacks: &Callbacks) -> Result<(), DominionError> {
        // Remove card from hand
        let c = self.hand.get(index).unwrap();
        if c.is_treasure() {
            let card = self.hand.remove(index).unwrap();
            card.effects_on_play(self, supply, other_players, callbacks);
            self.treasures_in_play.push_back(card.clone());

            Ok(())
        } else {
            Err(CardTypeMisMatch { expected: "Treasure".to_string() })
        }
    }

    pub fn play_all_treasures(&mut self, index: usize, supply: &mut Supply, other_players: &mut PlayerSlice, callbacks: &Callbacks) -> Result<(), DominionError> {
        for i in 0..self.hand.len() {
            let card = self.hand.get(index).unwrap();
            if card.is_treasure() {
                self.play_treasure(i, supply, other_players, callbacks)?;
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
        self.discard.append(&mut self.hand);
        self.discard.append(&mut self.actions_in_play);
        self.discard.append(&mut self.treasures_in_play);

        self.draw_cards(5);
    }

    /// Take a turn
    pub fn turn(&mut self, supply: &mut Supply, other_players: &mut PlayerSlice, callbacks: &Callbacks) {
        self.action_phase(supply, other_players, callbacks);
        self.buy_phase(supply, other_players, callbacks);
        self.cleanup();
    }

    /// Count up a player's victory points
    pub fn victory_points(&self) -> isize {
        let mut points = 0;
        for card in self.card_iter() {
            points += card.victory_points(self);
        }

        points
    }

    /// Prints out all cards in hand
    pub fn print_hand(&self) {
        let indent = "    ";

        println!("Hand:");
        print!("{}", indent);
        for card in &self.hand {
            print!("{}, ", card.name());
        }
        println!();
    }

    /// Prints out all cards in hand
    pub fn print_deck(&self) {
        let indent = "    ";

        println!("Deck:");
        print!("{}", indent);
        for card in &self.deck {
            print!("{}, ", card.name());
        }
        println!();
    }

    /// Prints out all cards in hand
    pub fn print_discard(&self) {
        let indent = "    ";

        println!("Discard:");
        print!("{}", indent);
        for card in &self.discard {
            print!("{}, ", card.name());
        }
    }

    /// Prints out all cards that the player has, in order, and where they are
    pub fn print_cards(&self) {
        self.print_hand();
        self.print_deck();
        self.print_discard();
        println!();
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
}
