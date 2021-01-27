//! Cards from the original Dominion set (2nd edition)
// TODO: provide brief documentation on all effects for each card just for convinience
// TODO: provide a String representation field in placeholder_card to represent the description of the card

use super::prelude::*;
use super::base::*;

card!(Artisan);
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

placeholder_card!(Bandit, "Bandit", 5);

placeholder_card!(Bureaucrat, "Bureaucrat", 4);

// Cellar
// +1 Action, discard any number of cards, then draw that many
card!(Cellar);
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

placeholder_card!(Chapel, "Chapel", 2);

//CouncilRoom
// +4 cards, 1 buy, each other player draws a card
card!(CouncilRoom);
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

placeholder_card!(Festival, "Festival", 5);

//Gardens
//effect: victory card, worth 1 per 10 cards you have(round down)
//placeholder_card!(Gardens, "Gardens", 4);
card!(Gardens);
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

placeholder_card!(Harbinger, "Harbinger", 3);

placeholder_card!(Laboratory, "Laboratory", 5);

placeholder_card!(Library, "Library", 5);

// Market
// effects: +1 Action, +1 Buy, +1 temp_coins, +1 Card
basic_action!(Market, "Market", 5, 1, 1, 1, 1);

placeholder_card!(Merchant, "Merchant", 3);

placeholder_card!(Militia, "Militia", 4);

placeholder_card!(Mine, "Mine", 5);

placeholder_card!(Moat, "Moat", 2);

placeholder_card!(Moneylender, "Moneylender", 4);

placeholder_card!(Poacher, "Poacher", 4);

placeholder_card!(Remodel, "Remodel", 4);

placeholder_card!(Sentry, "Sentry", 5);

placeholder_card!(Smithy, "Smithy", 4);

placeholder_card!(ThroneRoom, "Throne Room", 4);

placeholder_card!(Vassal, "Vassal", 3);

placeholder_card!(Village, "Village", 3);

placeholder_card!(Witch, "Witch", 5);

placeholder_card!(Workshop, "Workshop", 3);
