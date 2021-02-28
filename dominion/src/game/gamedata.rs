//! Defines the Game struct and its behavior

use serde::{Serialize, Deserialize};

use crate::game::prelude::*;
use crate::error::{DominionError::*, DominionResult};

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
        let mut supply: Supply = HashMap::new();

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


        for card in cards {
            // Check if we need Potions
            if card.potion_cost() > 0 {
                supply.insert(Box::new(Potion), 16);
            }

            // If card is victory card, count matches other victory cards
            // Otherwise use 10 copies
            let count = if card.is_victory() {
                victory_card_count
            } else {
                10
            };

            supply.insert(card, count);

        }

        Game::new_with_supply(player_count, supply)
    }

    /// Creates a new Game with a given supply and number of players
    pub fn new_with_supply(player_count: usize, supply: Supply) -> Game {
        let mut player_vec: PlayerList = Vec::with_capacity(player_count);
        let trash: CardDeck = VecDeque::new();
        let extras: Supply = HashMap::new();

        for i in 0..player_count {
            player_vec.push(Player::new_with_default_deck(i))
        }

        Game { players: player_vec, supply, trash, extras }
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

    /// Plays an action [card](Card) from the hand of the player corresponding
    /// to the given index
    ///
    /// This is the function to call when a player plays a card directly
    pub fn play_action_from_hand(&mut self, player_index: usize, card_index: usize, callbacks: &Callbacks) -> DominionResult {
        // Remove card from hand
        let player = &mut self.players[player_index];
        let card = player.hand.get(card_index).unwrap();
        if card.is_action() {
            let card = player.hand.remove(card_index).unwrap();
            player.in_play.push_back(card.clone());

            player.resources.actions -= 1;
            self.action_effects(player_index, card, callbacks);

            Ok(())
        } else {
            Err(CardTypeMisMatch { expected: Action })
        }
    }

    /// Gives the player the effects of an action card as if they had played it
    ///
    /// Does not subtract actions from the player's total. Should only be called
    /// in the effects() function of other cards (e.g. Throne Room)
    pub fn action_effects(&mut self, player_index: usize, card: Box<dyn Card>, callbacks: &Callbacks) {
        // Effects on the player who played the card
        card.effects_on_play(self, player_index, callbacks);

        // Attack effects, if any
        if card.is_attack() {
            let targets = self.get_targets(player_index, card.attack_targets().expect("Card has Attack type but does not define targets!"));

            for index in targets {
                card.attack_effects(self, index, callbacks)
            }
        }
    }

    /// Convert the attack target type into a vec of player indices
    pub fn get_targets(&mut self, player_index: usize, target_type: AttackTargetType) -> Vec<usize> {
        match target_type {
            EveryoneElse => {
                let mut indices = vec![];
                for i in 0..self.players.len() {
                    indices.push(i);
                }
                indices.remove(player_index);

                indices
            }

            PlayerToLeft => { vec![player_index + 1] }
        }
    }

    /// Take a turn
    pub fn turn(&mut self, player_index: usize, callbacks: &Callbacks) {
        let player = &mut self.players[player_index];

        player.reset_state();

        player.phase = Phase::ActionPhase;
        self.action_phase(player_index, callbacks);

        let player = &mut self.players[player_index];
        player.phase = Phase::BuyPhase;
        self.buy_phase(player_index, callbacks);

        let player = &mut self.players[player_index];
        player.phase = Phase::NightPhase;
        // TODO: Night phase

        let player = &mut self.players[player_index];
        player.phase = Phase::CleanupPhase;
        player.cleanup();

        player.phase = Phase::OutOfTurn;
    }

    /// Action phase
    pub fn action_phase(&mut self, player_index: usize, callbacks: &Callbacks) {
        //TODO (much later): Duration cards

        let mut more = true;
        while (self.players[player_index].resources.actions > 0) && more {
            let player = &mut self.players[player_index];

            more = (callbacks.prompt_player_done)();
            if more {
                for i in 0..player.hand.len() {
                    let card = player.hand.get(i).unwrap();
                    if card.is_action() {
                        self.play_action_from_hand(player_index, i, callbacks).unwrap();
                        break;
                    }
                }
            }
        }
    }

    /// Gain a copy of a card to the discard pile
    pub fn gain(&mut self, player_index: usize, card: Box<dyn Card>, callbacks: &Callbacks) -> DominionResult {
        if *self.supply.get(&card).unwrap() == 0 {
            return Err(EmptyPile{card});
        }

        *self.supply.get_mut(&card).unwrap() -= 1;
        card.effects_on_gain(self, player_index, callbacks);

        let player = &mut self.players[player_index];
        player.discard.push_back(card);
        Ok(())
    }

    /// Gain a copy of a card to hand
    pub fn gain_to_hand(&mut self, player_index: usize, card: Box<dyn Card>, callbacks: &Callbacks) -> DominionResult {
        if *self.supply.get(&card).unwrap() == 0 {
            return Err(EmptyPile{card});
        }

        *self.supply.get_mut(&card).unwrap() -= 1;
        card.effects_on_gain(self, player_index, callbacks);

        let player = &mut self.players[player_index];
        player.hand.push_back(card);
        Ok(())
    }

    /// Gain a copy of a card to the top of the deck
    pub fn gain_to_deck_top(&mut self, player_index: usize, card: Box<dyn Card>, callbacks: &Callbacks) -> DominionResult {
        if *self.supply.get(&card).unwrap() == 0 {
            return Err(EmptyPile{card});
        }

        *self.supply.get_mut(&card).unwrap() -= 1;
        card.effects_on_gain(self, player_index, callbacks);

        let player = &mut self.players[player_index];
        player.deck.push_front(card);
        Ok(())
    }

    /// Buy a card
    pub fn buy_card(&mut self, player_index: usize, card: Box<dyn Card>, callbacks: &Callbacks) -> DominionResult {
        let player = &mut self.players[player_index];

        if player.resources.coins_remaining < card.coin_cost() {
            return Err(InsufficientFunds);
        }

        card.effects_on_buy(self, player_index, callbacks);
        card.effects_on_gain(self, player_index, callbacks);

        self.gain(player_index, card.clone(), callbacks)?;

        let player = &mut self.players[player_index];
        player.resources.temp_coins -= card.coin_cost();

        player.resources.buys -= 1;
        Ok(())
    }

    /// Buy phase
    pub fn buy_phase(&mut self, player_index: usize, callbacks: &Callbacks) {
        let player = &mut self.players[player_index];

        // TODO: prompt user to play treasures
        player.resources.coins_remaining = player.resources.coins_in_hand + player.resources.temp_coins;
        player.resources.potions_remaining = player.resources.potions_in_hand + player.resources.temp_potions;

        if player.resources.debt > 0 {
            // Can't buy
            // TODO: prompt player to acknowledge paying off debt
            // If outstanding debt exists after paying, player cannot buy
            if player.resources.coins_remaining < player.resources.debt {
                player.resources.debt -= player.resources.coins_remaining;
                return;
            } else {
                player.resources.coins_remaining -= player.resources.debt;
                player.resources.debt = 0;
            }
        }

        let mut more = true;
        while (self.players[player_index].resources.buys > 0) && more {
            // Buy cards
            let mut card = (callbacks.choose_card_from_supply)(&self.supply);

            // If player chooses a card they cannot buy, loop
            while self.buy_card(player_index, card.clone(), callbacks).is_err() {
                card = (callbacks.choose_card_from_supply)(&self.supply);
            }

            more = (callbacks.prompt_player_done)();
        }
    }

    pub fn play_treasure(&mut self, player_index: usize, card_index: usize, callbacks: &Callbacks) -> DominionResult {
        let player = &mut self.players[player_index];

        // Remove card from hand
        let c = player.hand.get(card_index).unwrap();
        if c.is_treasure() {
            let card = player.hand.remove(card_index).unwrap();
            card.effects_on_play(self, player_index, callbacks);
            let player = &mut self.players[player_index];
            player.in_play.push_back(card.clone());

            Ok(())
        } else {
            Err(CardTypeMisMatch { expected: Treasure })
        }
    }

    pub fn play_all_treasures(&mut self, player_index: usize, callbacks: &Callbacks) -> DominionResult {
        let range = self.players[player_index].hand.len();

        for i in 0..range {
            let player = &mut self.players[player_index];
            let card = player.hand.get(i).unwrap();
            if card.is_treasure() {
                self.play_treasure(player_index, i, callbacks)?;
            }
        }

        Ok(())
    }

    /// Returns vector of available cards with cost less than or equal
    /// to the given value
    ///
    /// Hopefully we can combine this and related methods into one generic one
    pub fn return_avail_cards_ucost(&self, cost: usize) -> CardList {
        //TODO: rewrite to not use collect and to use filter() with the lambda passed in
        return self.supply.keys()
            .filter(|a| *self.supply.get(*a).unwrap() > 0)
            .filter(|a| a.coin_cost() <= cost)
            .cloned()
            .collect();
    }

    /// Returns vector of available cards with cost greater than or equal
    /// to the given value
    ///
    /// Hopefully we can combine this and related methods into one generic one
    pub fn return_avail_cards_acost(&self, cost: usize) -> CardList {
        //TODO: rewrite to not use collect and to use filter() with the lambda passed in
        return self.supply.keys()
            .filter(|a| *self.supply.get(*a).unwrap() > 0)
            .filter(|a| a.coin_cost() >= cost)
            .cloned()
            .collect();
    }

    /// Returns vector of cards available of a certain type
    ///
    /// Hopefully we can combine this and related methods into one generic one
    pub fn return_avail_cards_type(&self, t: CardType) -> CardList {
        //TODO: rewrite to not use collect and to use filter() with the lambda passed in
        return self.supply.keys()
            .filter(|a| *self.supply.get(*a).unwrap() > 0)
            .filter(|a| a.types().contains(&t))
            .cloned()
            .collect();
    }
}
