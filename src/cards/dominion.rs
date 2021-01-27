//! Cards from the original Dominion set (2nd edition)
// TODO: provide brief documentation on all effects for each card just for convinience
// TODO: provide a String representation field in placeholder_effects to represent the description of the card

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
    fn effects_on_play(&self, player: &mut Player, supply: &mut Supply, other_players: &mut PlayerSlice, callbacks: &Callbacks) {
        // TODO: change to card of choice from supply and put a card from hand back on deck
        let card = Box::new(Silver);
        player.gain_to_hand(card, supply, other_players, callbacks);
    }
}

/// [Wiki link](http://wiki.dominionstrategy.com/index.php/Bandit)
#[derive(Clone, Serialize, Deserialize)]
pub struct Bandit;

placeholder_effects!(Bandit, "Bandit", 5);

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
    fn effects_on_play(&self, player: &mut Player, _: &mut Supply, _: &mut PlayerSlice, callbacks: &Callbacks) {
        let indexes: Vec<usize> = (callbacks.prompt_indices_from_hand)();
        let count = indexes.len();

        player.discard_given_indexes(indexes);
        player.draw_cards(count);
    }
}

/// [Wiki link](http://wiki.dominionstrategy.com/index.php/Chapel)
#[derive(Clone, Serialize, Deserialize)]
pub struct Chapel;

placeholder_effects!(Chapel, "Chapel", 2);

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
    fn effects_on_play(&self, player: &mut Player, _supply: &mut Supply, other_players: &mut PlayerSlice, _: &Callbacks) {
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

placeholder_effects!(Festival, "Festival", 5);

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

placeholder_effects!(Harbinger, "Harbinger", 3);

/// [Wiki link](http://wiki.dominionstrategy.com/index.php/Laboratory)
#[derive(Clone, Serialize, Deserialize)]
pub struct Laboratory;

placeholder_effects!(Laboratory, "Laboratory", 5);

/// [Wiki link](http://wiki.dominionstrategy.com/index.php/Library)
#[derive(Clone, Serialize, Deserialize)]
pub struct Library;

placeholder_effects!(Library, "Library", 5);

// Market
// effects: +1 Action, +1 Buy, +1 temp_coins, +1 Card
basic_action!(Market, "Market", 5, 1, 1, 1, 1);

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

placeholder_effects!(Smithy, "Smithy", 4);

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
