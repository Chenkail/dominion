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
    fn effects_on_play(&self, player: &mut Player, supply: &mut Supply, other_players: &PlayerSlice) {
        // TODO: change to card of choice from supply and put a card from hand back on deck
        let card = Box::new(Silver);
        player.gain_to_hand(card, supply, other_players);
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
    fn effects_on_play(&self, player: &mut Player, supply: &mut Supply, other_players: &PlayerSlice) {
        let num_discard: i32 = 3; // 3 is placeholder number, we ideally want to prompt the player through callbacks for this value
        let indexes: Vec<usize> = Vec::new(); 
        for i in 0..num_discard {
            //prompt player for indexes to discard
        }

        //
        player.discard_given_indexes(indexes);
        player.draw_cards(num_discard);
    }
}

placeholder_card!(Chapel, "Chapel", 2);

placeholder_card!(CouncilRoom, "Council Room", 5);

placeholder_card!(Festival, "Festival", 5);

placeholder_card!(Gardens, "Gardens", 4);

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
