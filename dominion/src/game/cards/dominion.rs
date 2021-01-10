/// Cards from the original Dominion set (2nd edition?)

use crate::game::{player::Player, traits::*};

pub struct Smithy;
impl Card for Smithy {
    fn cost(&self) -> i32 {
        return 4;
    }
    
    fn name(&self) -> &'static str {
        return "Smithy";
    }
}
impl Action for Smithy {
    fn effects(&self, player: &mut Player) {
        player.draw_cards(3);
    }
}
