//! Cards from the original Dominion set (2nd edition)
// TODO: provide brief documentation on all effects for each card just for convenience
// TODO: Add description fn for cards that have it

use super::prelude::*;
use super::base::*;

/// [Wiki link](http://wiki.dominionstrategy.com/index.php/Artisan)
#[derive(Clone, Serialize, Deserialize)]
pub struct Artisan;

#[typetag::serde]
impl Card for Artisan {
    name!("Artisan");
    cost!(6);
    types!(vec![Action]);
    fn effects_on_play(&self, player: &mut Player, supply: &mut Supply, _trash: &mut CardDeck, other_players: &mut PlayerSlice, callbacks: &Callbacks) {
        // TODO: change to card of choice from supply and put a card from hand back on deck
        let card = Box::new(Silver);
        player.gain_to_hand(card, supply, other_players, callbacks);
    }
}

/// [Wiki link](http://wiki.dominionstrategy.com/index.php/Bandit)
#[derive(Clone, Serialize, Deserialize)]
pub struct Bandit;

#[typetag::serde]
impl Card for Bandit {
    name!("Bandit");
    cost!(5);
    types!(vec![Action, Attack]);
    fn effects_on_play(&self, player: &mut Player, supply: &mut Supply, trash: &mut CardDeck, other_players: &mut PlayerSlice, callbacks: &Callbacks) {
        player.gain(Box::new(Gold), supply, other_players, callbacks);

        for p in other_players {
            //callback to reveal top 2 cards in their hand
            
            // we need more callbacks? I'll think about what to do here for incredibly specific card descs
            // we want to be able to send a list of allowed indexes to the user to pick from here.
            let indexes: Vec<usize> = (callbacks.prompt_indices_from_hand)();
            
            
            p.trash_given_indexes(indexes, trash)

        }
    }
}

/// [Wiki link](http://wiki.dominionstrategy.com/index.php/Bureaucrat)
#[derive(Clone, Serialize, Deserialize)]
pub struct Bureaucrat;

placeholder_effects!(Bureaucrat, "Bureaucrat", 4);

// Cellar
// +1 Action, discard any number of cards, then draw that many
/// [Wiki link](http://wiki.dominionstrategy.com/index.php/Cellar)
#[derive(Clone, Serialize, Deserialize)]
pub struct Cellar;

#[typetag::serde]
impl Card for Cellar {
    name!("Cellar");
    cost!(2);
    types!(vec![Action]);
    fn effects_on_play(&self, player: &mut Player, _supply: &mut Supply, _trash: &mut CardDeck, _other_players: &mut PlayerSlice, callbacks: &Callbacks) {
        let indexes: Vec<usize> = (callbacks.prompt_indices_from_hand)();
        let count = indexes.len();

        player.discard_given_indexes(indexes);
        player.draw_cards(count);
    }
}

/// [Wiki link](http://wiki.dominionstrategy.com/index.php/Chapel)
#[derive(Clone, Serialize, Deserialize)]
pub struct Chapel;
#[typetag::serde]
impl Card for Chapel {
    name!("Chapel");
    cost!(2);
    types!(vec![Action]);

    fn effects_on_play(&self, player: &mut Player, _supply: &mut Supply, trash: &mut CardDeck, _other_players: &mut PlayerSlice, callbacks: &Callbacks) {
        let indexes: Vec<usize> = (callbacks.prompt_indices_from_hand_u)(4);
        player.trash_given_indexes(indexes, trash);
    }

}

//placeholder_effects!(Chapel, "Chapel", 2);

// Council Room
// +4 cards, +1 buy, each other player draws a card
/// [Wiki link](http://wiki.dominionstrategy.com/index.php/Council_Room)
#[derive(Clone, Serialize, Deserialize)]
pub struct CouncilRoom;

#[typetag::serde]
impl Card for CouncilRoom {
    name!("Council Room");
    cost!(5);
    types!(vec![Action]);
    fn effects_on_play(&self, player: &mut Player, _supply: &mut Supply, _trash: &mut CardDeck, other_players: &mut PlayerSlice, _callbacks: &Callbacks) {
        player.draw_cards(4);
        player.add_buys(1);

        for p in other_players {
            p.draw_cards(1);
        }
    }
}

/// [Wiki link](http://wiki.dominionstrategy.com/index.php/Festival)
#[derive(Clone, Serialize, Deserialize)]
pub struct Festival;

#[typetag::serde]
impl Card for Festival {
    name!("Festival");
    cost!(5);
    types!(vec![Action]);
    basic_action_effects!(0, 2, 1, 2);
}

// Gardens
// effect: victory card, worth 1 per 10 cards you have(round down)
/// [Wiki link](http://wiki.dominionstrategy.com/index.php/Gardens)
#[derive(Clone, Serialize, Deserialize)]
pub struct Gardens;

#[typetag::serde]
impl Card for Gardens {
    name!("Gardens");
    cost!(4);
    types!(vec![Victory]);

    //integer division should be fine
    fn victory_points(&self, player: &Player) -> isize {
        ((player.deck.len() + player.hand.len() + player.discard.len()) / 10) as isize
    }
}

/// [Wiki link](http://wiki.dominionstrategy.com/index.php/Harbinger)
#[derive(Clone, Serialize, Deserialize)]
pub struct Harbinger;

#[typetag::serde]
impl Card for Harbinger {
    name!("Harbringer");
    cost!(3);
    types!(vec![Action]);
    fn effects_on_play(&self, player: &mut Player, _supply: &mut Supply, _trash: &mut CardDeck, _other_players: &mut PlayerSlice, callbacks: &Callbacks) {
        player.add_actions(1);
        player.draw_cards(1);

        //look through discard and pick
        (callbacks.reveal_top_discard_pile)(player, player.discard.len());

        //TODO:
        //create callback for prompt_indexes from discard
        //create method for moving from discard to hand

    }
}

/// [Wiki link](http://wiki.dominionstrategy.com/index.php/Laboratory)
#[derive(Clone, Serialize, Deserialize)]
pub struct Laboratory;

#[typetag::serde]
impl Card for Laboratory {
    name!("Laboratory");
    cost!(5);
    types!(vec![Action]);
    basic_action_effects!(2, 1, 0, 0);
}

/// [Wiki link](http://wiki.dominionstrategy.com/index.php/Library)
#[derive(Clone, Serialize, Deserialize)]
pub struct Library;

placeholder_effects!(Library, "Library", 5);

// Market
// effects: +1 Action, +1 Buy, +1 temp_coins, +1 Card
/// [Wiki link](http://wiki.dominionstrategy.com/index.php/Market)
#[derive(Clone, Serialize, Deserialize)]
pub struct Market;

#[typetag::serde]
impl Card for Market {
    name!("Market");
    cost!(5);
    types!(vec![Action]);
    basic_action_effects!(1, 1, 1, 1);
}


/// [Wiki link](http://wiki.dominionstrategy.com/index.php/Merchant)
#[derive(Clone, Serialize, Deserialize)]
pub struct Merchant;

placeholder_effects!(Merchant, "Merchant", 3);

/// [Wiki link](http://wiki.dominionstrategy.com/index.php/Militia)
#[derive(Clone, Serialize, Deserialize)]
pub struct Militia;

placeholder_effects!(Militia, "Militia", 4);

/// [Wiki link](http://wiki.dominionstrategy.com/index.php/Mine)
#[derive(Clone, Serialize, Deserialize)]
pub struct Mine;

placeholder_effects!(Mine, "Mine", 5);

/// [Wiki link](http://wiki.dominionstrategy.com/index.php/Moat)
#[derive(Clone, Serialize, Deserialize)]
pub struct Moat;

placeholder_effects!(Moat, "Moat", 2);

/// [Wiki link](http://wiki.dominionstrategy.com/index.php/Moneylender)
#[derive(Clone, Serialize, Deserialize)]
pub struct Moneylender;

placeholder_effects!(Moneylender, "Moneylender", 4);

/// [Wiki link](http://wiki.dominionstrategy.com/index.php/Poacher)
#[derive(Clone, Serialize, Deserialize)]
pub struct Poacher;

placeholder_effects!(Poacher, "Poacher", 4);

/// [Wiki link](http://wiki.dominionstrategy.com/index.php/Remodel)
#[derive(Clone, Serialize, Deserialize)]
pub struct Remodel;

placeholder_effects!(Remodel, "Remodel", 4);

/// [Wiki link](http://wiki.dominionstrategy.com/index.php/Sentry)
#[derive(Clone, Serialize, Deserialize)]
pub struct Sentry;

placeholder_effects!(Sentry, "Sentry", 5);

/// [Wiki link](http://wiki.dominionstrategy.com/index.php/Smithy)
#[derive(Clone, Serialize, Deserialize)]
pub struct Smithy;

#[typetag::serde]
impl Card for Smithy {
    name!("Smithy");
    cost!(4);
    types!(vec![Action]);
    basic_action_effects!(3, 0, 0, 0);
}

/// [Wiki link](http://wiki.dominionstrategy.com/index.php/ThroneRoom)
#[derive(Clone, Serialize, Deserialize)]
pub struct ThroneRoom;

placeholder_effects!(ThroneRoom, "Throne Room", 4);

/// [Wiki link](http://wiki.dominionstrategy.com/index.php/Vassal)
#[derive(Clone, Serialize, Deserialize)]
pub struct Vassal;

placeholder_effects!(Vassal, "Vassal", 3);

/// [Wiki link](http://wiki.dominionstrategy.com/index.php/Village)
#[derive(Clone, Serialize, Deserialize)]
pub struct Village;

placeholder_effects!(Village, "Village", 3);

/// [Wiki link](http://wiki.dominionstrategy.com/index.php/Witch)
#[derive(Clone, Serialize, Deserialize)]
pub struct Witch;

placeholder_effects!(Witch, "Witch", 5);

/// [Wiki link](http://wiki.dominionstrategy.com/index.php/Workshop)
#[derive(Clone, Serialize, Deserialize)]
pub struct Workshop;

placeholder_effects!(Workshop, "Workshop", 3);
