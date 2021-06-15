//! Defines the Game struct and its behavior

use serde::{Serialize, Deserialize};

use crate::prelude::*;

/// The data for a game of Dominion.

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct Game {
    pub started: bool,
    pub current_turn: usize,
    pub players: PlayerList,
    pub supply: Supply,
    pub trash: CardDeck,
    pub extras: Supply,
}

impl Game {
    pub fn default_supply_list() -> CardList {
        card_vec![Cellar, Market, Merchant, Militia, Mine, Moat, Remodel, Smithy, Village, Workshop]
    }

    pub fn new() -> Game {
        Game::default()
    }

    pub fn player_count(&self) -> usize {
        self.players.len()
    }

    fn hand_sizes(&self) -> Vec<usize> {
        self.players
            .clone()
            .into_iter()
            .map(|player| player.hand_size())
            .collect()
    }

    pub fn partial_game(&self, player_number: usize) -> PartialGame {
        PartialGame {
            current_turn: self.current_turn,
            player: self.players[player_number].clone(),
            hand_sizes: self.hand_sizes(),
            supply: self.supply.clone(),
            trash: self.trash.clone(),
            extras: self.extras.clone(),
        }
    }

    /// Generates the supply piles for the game given a list of cards to use
    pub fn generate_supply(&mut self, cards: CardList) -> DominionResult {
        let player_count = self.player_count();

        let (victory_card_count, province_count, curse_count) = match player_count {
            2 => (8, 8, 10),
            3 => (12, 12, 20),
            4 => (12, 12, 30),
            5 => (12, 15, 40),
            6 => (12, 18, 50),
            _ => return Err(NotEnoughPlayers)
        };

        let mut supply: Supply = HashMap::new();
        supply.insert(Copper.name().to_string(), SupplyEntry { card: Box::new(Copper), count: 40 });
        supply.insert(Silver.name().to_string(), SupplyEntry { card: Box::new(Silver), count: 40 });
        supply.insert(Gold.name().to_string(), SupplyEntry { card: Box::new(Gold), count: 40 });
        supply.insert(Estate.name().to_string(), SupplyEntry { card: Box::new(Estate), count: victory_card_count });
        supply.insert(Duchy.name().to_string(), SupplyEntry { card: Box::new(Duchy), count: victory_card_count });
        supply.insert(Province.name().to_string(), SupplyEntry { card: Box::new(Province), count: victory_card_count });
        supply.insert(BasicCurse.name().to_string(), SupplyEntry { card: Box::new(BasicCurse), count: victory_card_count });


        for card in cards {
            // Check if we need Potions
            if card.potion_cost() > 0 {
                supply.insert(Potion.name().to_string(), SupplyEntry { card: Box::new(Potion), count: 16 });
            }

            // If card is victory card, count matches other victory cards
            // Otherwise use 10 copies
            let count = if card.is_victory() {
                victory_card_count
            } else {
                10
            };

            supply.insert(card.name().to_string(), SupplyEntry{ card, count });
        }

        self.supply = supply;
        Ok(())
    }

    /// Add a player to the game
    pub fn add_player(&mut self, player: Player) {
        self.players.push(player);
    }

    /// Prints out all the cards in the supply and their remaining quantities
    pub fn print_supply(&mut self) {
        for entry in self.supply.values() {
            println!("{}: {} cards", entry.card, entry.count);
        }
    }

    /// checks game state, used for victory condition.
    /// returns true if the province cards are exhausted OR when 3 stacks in the supply are exhausted.
    pub fn victory_met(&mut self) -> bool {
        let province: Box<dyn Card> = Box::new(Province);
        if self.supply.get(province.name()).unwrap().count == 0 {
            return true;
        }

        if self.supply
            .values()
            .filter(|e| e.count == 0)
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

            self.check_reactions(player_index, OtherPlayerPlaysAttack, callbacks);

            for index in targets {
                let player = &self.players[player_index];
                let champion: Box<dyn Card> = Box::new(crate::cards::adventures::Champion);

                if !(player.state.immune || player.in_play.contains(&champion)) {
                    card.attack_effects(self, index, callbacks)
                }

                let mut player = &mut self.players[player_index];
                player.state.immune = false;
            }
        }
    }

    /// Convert the attack target type into a vec of player indices
    pub fn get_targets(&mut self, player_index: usize, target_type: AttackTargetType) -> Vec<usize> {
        match target_type {
            EveryoneElse => {
                let mut indices = vec![];
                for i in 0..self.player_count() {
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
        if self.supply.get(card.name()).unwrap().count == 0 {
            return Err(EmptyPile{card});
        }

        self.supply.get_mut(card.name()).unwrap().count -= 1;
        card.effects_on_gain(self, player_index, callbacks);

        let player = &mut self.players[player_index];
        player.discard.push_back(card);
        Ok(())
    }

    /// Gain a copy of a card to hand
    pub fn gain_to_hand(&mut self, player_index: usize, card: Box<dyn Card>, callbacks: &Callbacks) -> DominionResult {
        if self.supply.get(card.name()).unwrap().count == 0 {
            return Err(EmptyPile{card});
        }

        self.supply.get_mut(card.name()).unwrap().count -= 1;
        card.effects_on_gain(self, player_index, callbacks);

        let player = &mut self.players[player_index];
        player.hand.push_back(card);
        Ok(())
    }

    /// Gain a copy of a card to the top of the deck
    pub fn gain_to_deck_top(&mut self, player_index: usize, card: Box<dyn Card>, callbacks: &Callbacks) -> DominionResult {
        if self.supply.get(card.name()).unwrap().count == 0 {
            return Err(EmptyPile{card});
        }

        self.supply.get_mut(card.name()).unwrap().count -= 1;
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

        if self.supply.get(card.name()).unwrap().count == 0 {
            return Err(EmptyPile { card });
        }

        card.effects_on_buy(self, player_index, callbacks);
        card.effects_on_gain(self, player_index, callbacks);

        self.gain(player_index, card.clone(), callbacks)?;

        let player = &mut self.players[player_index];
        player.resources.temp_coins -= card.coin_cost();

        player.resources.buys -= 1;

        // Hovel check
        if card.is_victory() {
            self.check_reactions(player_index, BuyAVictoryCard, callbacks);
        }

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

    pub fn check_reactions(&mut self, player_index: usize, reaction_trigger: ReactionTrigger, callbacks: &Callbacks) {
        // TODO: prompt player and perform reaction

    }

    /// Returns vector of available cards with cost less than or equal
    /// to the given value
    ///
    /// Hopefully we can combine this and related methods into one generic one
    pub fn return_avail_cards_ucost(&self, cost: usize) -> CardList {
        //TODO: rewrite to not use collect and to use filter() with the lambda passed in
        return self.supply.values()
            .filter(|entry| entry.count > 0)
            .filter(|entry| entry.card.coin_cost() <= cost)
            .map(|entry| entry.card.clone())
            .collect();
    }

    /// Returns vector of available cards with cost greater than or equal
    /// to the given value
    ///
    /// Hopefully we can combine this and related methods into one generic one
    pub fn return_avail_cards_acost(&self, cost: usize) -> CardList {
        //TODO: rewrite to not use collect and to use filter() with the lambda passed in
        return self.supply.values()
            .filter(|entry| entry.count > 0)
            .filter(|entry| entry.card.coin_cost() >= cost)
            .map(|entry| entry.card.clone())
            .collect();
    }

    /// Returns vector of cards available of a certain type
    ///
    /// Hopefully we can combine this and related methods into one generic one
    pub fn return_avail_cards_type(&self, t: CardType) -> CardList {
        //TODO: rewrite to not use collect and to use filter() with the lambda passed in
        return self.supply.values()
            .filter(|entry| entry.count > 0)
            .filter(|entry| entry.card.types().contains(&t))
            .map(|entry| entry.card.clone())
            .collect();
    }
}
