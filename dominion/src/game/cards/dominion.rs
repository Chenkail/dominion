/// Cards from the original Dominion set (2nd edition?)

use crate::game::{player::Player, traits::*};
use dominion_macros::*;

card!(Smithy, "Smithy", 4);
impl Action for Smithy {
    fn effects(&self, player: &mut Player) {
        player.draw_cards(3);
    }
}

card!(Market, "Market", 5);
impl Action for Market {
    fn effects(&self, player: &mut Player) {
        player.draw_cards(1);
        player.add_actions(1);
        player.add_buys(1);
        player.add_coins(1);
    }
}
