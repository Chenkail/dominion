//! Cards from the original Dominion set (2nd edition)
// TODO: provide brief documentation on all effects for each card just for convenience
// TODO: Add description fn for cards that have it

use super::prelude::*;
use super::base::*;

card!(Artisan, "[Wiki link](http://wiki.dominionstrategy.com/index.php/Artisan)");
#[typetag::serde]
impl Card for Artisan {
    name!("Artisan");
    card_cost!(6);
    types!(vec![Action]);
    fn effects_on_play(&self, game: &mut Game, player_index: usize, callbacks: &Callbacks) {
        // TODO: change to card of choice from supply and put a card from hand back on deck
        let card = Box::new(Silver);
        let result = game.gain_to_hand(player_index, card, callbacks);
        if result.is_err() {
            // TODO: get new card
        }
    }
}


card!(Bandit, "[Wiki link](http://wiki.dominionstrategy.com/index.php/Bandit)");
#[typetag::serde]
impl Card for Bandit {
    name!("Bandit");
    card_cost!(5);
    types!(vec![Action, Attack]);

    fn effects_on_play(&self, game: &mut Game, player_index: usize, callbacks: &Callbacks) {
        let _ = game.gain(player_index, Box::new(Gold), callbacks);
    }

    fn attack_targets(&self) -> Option<AttackTargetType> { Some(EveryoneElse) }

    fn attack_effects(&self, game: &mut Game, player_index: usize, callbacks: &Callbacks) {
        let player = &mut game.players[player_index];
        //callback to reveal top 2 cards in their hand

        // we need more callbacks? I'll think about what to do here for
        // incredibly specific card descriptions. we want to be able to
        // send a list of allowed indexes to the user to pick from here.
        let indexes: Vec<usize> = (callbacks.prompt_indices_from_hand)();
        player.trash_given_indexes(indexes, &mut game.trash);
    }
}


card!(Bureaucrat, "[Wiki link](http://wiki.dominionstrategy.com/index.php/Bureaucrat)");
placeholder_effects!(Bureaucrat, "Bureaucrat", 4);

// Cellar
// +1 Action, discard any number of cards, then draw that many
card!(Cellar, "[Wiki link](http://wiki.dominionstrategy.com/index.php/Cellar)");
#[typetag::serde]
impl Card for Cellar {
    name!("Cellar");
    card_cost!(2);
    types!(vec![Action]);
    fn effects_on_play(&self, game: &mut Game, player_index: usize, callbacks: &Callbacks) {
        let player = &mut game.players[player_index];
        let indexes: Vec<usize> = (callbacks.prompt_indices_from_hand)();
        let count = indexes.len();

        player.discard_given_indexes(indexes);
        player.draw_cards(count);
    }
}

card!(Chapel, "[Wiki link](http://wiki.dominionstrategy.com/index.php/Chapel)");
#[typetag::serde]
impl Card for Chapel {
    name!("Chapel");
    card_cost!(2);
    types!(vec![Action]);

    fn effects_on_play(&self, game: &mut Game, player_index: usize, callbacks: &Callbacks) {
        let player = &mut game.players[player_index];
        let indexes: Vec<usize> = (callbacks.prompt_indices_from_hand_u)(4);
        player.trash_given_indexes(indexes, &mut game.trash);
    }
}


// Council Room
// +4 cards, +1 buy, each other player draws a card
card!(CouncilRoom, "[Wiki link](http://wiki.dominionstrategy.com/index.php/Council_Room)");
#[typetag::serde]
impl Card for CouncilRoom {
    name!("Council Room");
    card_cost!(5);
    types!(vec![Action]);
    fn effects_on_play(&self, game: &mut Game, player_index: usize, _: &Callbacks) {
        let player = &mut game.players[player_index];
        player.draw_cards(4);
        player.add_buys(1);

        let player_count = game.players.len();

        for i in 1..player_count {
            let index = (i + player_index) % player_count;
            let player = &mut game.players[index];
            player.draw_cards(1);
        }
    }
}

basic_action!(
    Festival,
    "Festival",
    cost=5,
    cards=0,
    actions=2,
    buys=1,
    coins=2,
    doc="[Wiki link](http://wiki.dominionstrategy.com/index.php/Festival)");

// Gardens
//
// Effect: victory card, worth 1 per 10 cards you have(round down)
card!(Gardens, "[Wiki link](http://wiki.dominionstrategy.com/index.php/Gardens)");
#[typetag::serde]
impl Card for Gardens {
    name!("Gardens");
    card_cost!(4);
    types!(vec![Victory]);

    //integer division should be fine
    fn victory_points(&self, player: &Player) -> isize {
        ((player.deck.len() + player.hand.len() + player.discard.len()) / 10) as isize
    }
}

card!(Harbinger, "[Wiki link](http://wiki.dominionstrategy.com/index.php/Harbinger)");
#[typetag::serde]
impl Card for Harbinger {
    name!("Harbinger");
    card_cost!(3);
    types!(vec![Action]);
    fn effects_on_play(&self, game: &mut Game, player_index: usize, callbacks: &Callbacks) {
        let player = &mut game.players[player_index];
        player.add_actions(1);
        player.draw_cards(1);

        //look through discard and pick
        (callbacks.reveal_top_discard_pile)(player, player.discard.len());

        //TODO:
        //create callback for prompt_indexes from discard
        let indexes = vec![0, 1, 2];
        //create method for moving from discard to hand
        player.move_discard_given_indexes_to_hand(indexes)
    }
}

card!(Laboratory, "[Wiki link](http://wiki.dominionstrategy.com/index.php/Laboratory)");
#[typetag::serde]
impl Card for Laboratory {
    name!("Laboratory");
    card_cost!(5);
    types!(vec![Action]);
    basic_on_play_effects!(
        cards=2,
        actions=1,
        buys=0,
        coins=0);
}

card!(Library, "[Wiki link](http://wiki.dominionstrategy.com/index.php/Library)");
#[typetag::serde]
impl Card for Library {
    name!("Library");
    card_cost!(5);
    types!(vec![Action]);

    fn effects_on_play(&self, game: &mut Game, player_index: usize, callbacks: &Callbacks) {
        let player = &mut game.players[player_index];
        while player.hand.len() < 7 {
            if player.deck.front().unwrap().is_action() {
                //TODO: get player consent to draw or discard the card
                println!("discard? ");
                if (callbacks.get_player_consent)(player) {
                    player.discard.push_back(player.deck.pop_front().unwrap());
                }
            } else {
                player.draw_cards(1);
            }
        }
    }
}

// Market
// effects: +1 Action, +1 Buy, +1 temp_coins, +1 Card
basic_action!(
    Market,
    "Market",
    cost=5,
    cards=1,
    actions=1,
    buys=1,
    coins=1,
    doc="[Wiki link](http://wiki.dominionstrategy.com/index.php/Market)");

// Merchant
card!(Merchant, "[Wiki link](http://wiki.dominionstrategy.com/index.php/Merchant)");
#[typetag::serde]
impl Card for Merchant {
    name!("Merchant");
    card_cost!(3);
    types!(vec![Action]);
    fn effects_on_play(&self, game: &mut Game, player_index: usize, _: &Callbacks) {
        let p = game.players.get_mut(player_index).unwrap();
        p.add_actions(1);
        p.draw_cards(1);

        //TODO: add method on game
    }
}

card!(Militia, "[Wiki link](http://wiki.dominionstrategy.com/index.php/Militia)");
placeholder_effects!(Militia, "Militia", 4);


card!(Mine, "[Wiki link](http://wiki.dominionstrategy.com/index.php/Mine)");
placeholder_effects!(Mine, "Mine", 5);


card!(Moat, "[Wiki link](http://wiki.dominionstrategy.com/index.php/Moat)");
#[typetag::serde]
impl Card for Moat {
    name!("Moat");
    card_cost!(2);
    types!(vec![Action, Reaction]);
    fn effects_on_play(&self, game: &mut Game, player_index: usize, _: &Callbacks) {
        let p = game.players.get_mut(player_index).unwrap();
        p.draw_cards(2);
    }

    fn effects_on_react(&self, game: &mut Game, player_index: usize, _: &Callbacks) {
        // TODO: Fix this to make it a choice per attack, rather than making
        // the player completely immune until their next turn
        let p = game.players.get_mut(player_index).unwrap();
        p.state.immune = true;
    }

    fn reaction_trigger(&self) -> Option<ReactionTrigger> {
        Some(OtherPlayerPlaysAttack)
    }
}

card!(Moneylender, "[Wiki link](http://wiki.dominionstrategy.com/index.php/Moneylender)");
placeholder_effects!(Moneylender, "Moneylender", 4);


card!(Poacher, "[Wiki link](http://wiki.dominionstrategy.com/index.php/Poacher)");
placeholder_effects!(Poacher, "Poacher", 4);

card!(Remodel, "[Wiki link](http://wiki.dominionstrategy.com/index.php/Remodel)");
placeholder_effects!(Remodel, "Remodel", 4);

card!(Sentry, "[Wiki link](http://wiki.dominionstrategy.com/index.php/Sentry)");
placeholder_effects!(Sentry, "Sentry", 5);

basic_action!(
    Smithy, 
    "Smithy",
    cost=4,
    cards=3,
    actions=0,
    buys=0,
    coins=0,
    doc="[Wiki link](http://wiki.dominionstrategy.com/index.php/Smithy)");

card!(ThroneRoom, "[Wiki link](http://wiki.dominionstrategy.com/index.php/Throne_Room)");
#[typetag::serde]
impl Card for ThroneRoom {
    name!("Throne Room");
    card_cost!(4);
    types!(vec![Action]);

    fn effects_on_play(&self, game: &mut Game, player_index: usize, callbacks: &Callbacks) {
        let card_index = (callbacks.prompt_card_from_hand)();
        let player = &mut game.players[player_index];
        let card = player.hand.remove(card_index).unwrap();

        if !card.is_action() {
            // TODO: Prompt for new card
        }

        game.action_effects(player_index, card.clone(), callbacks);
        game.action_effects(player_index, card.clone(), callbacks);

        let player = &mut game.players[player_index];
        player.in_play.push_back(card);
    }
}


card!(Vassal, "[Wiki link](http://wiki.dominionstrategy.com/index.php/Vassal)");
placeholder_effects!(Vassal, "Vassal", 3);

basic_action!(Village, "Village",
            cost=3,
            cards=1,
            actions=2,
            buys=0,
            coins=0,
            doc="[Wiki link](http://wiki.dominionstrategy.com/index.php/Village)");

card!(Witch, "[Wiki link](http://wiki.dominionstrategy.com/index.php/Witch)");
#[typetag::serde]
impl Card for Witch {
    name!("Witch");
    card_cost!(5);
    types!(vec![Action, Attack]);
    basic_on_play_effects!(cards=2, actions=0, buys=0, coins=0);

    fn attack_targets(&self) -> Option<AttackTargetType> { Some(EveryoneElse) }

    fn attack_effects(&self, game: &mut Game, player_index: usize, callbacks: &Callbacks) {
        let _ = game.gain(player_index, Box::new(BasicCurse), callbacks);
    }
}


card!(Workshop, "[Wiki link](http://wiki.dominionstrategy.com/index.php/Workshop)");
#[typetag::serde]
impl Card for Workshop {
    name!("Workshop");
    card_cost!(3);
    types!(vec![Action]);

    fn effects_on_play(&self, game: &mut Game, player_index: usize, callbacks: &Callbacks) {
        let potential_cards = game.return_avail_cards_ucost(4);
    }
}
