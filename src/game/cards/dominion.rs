use crate::game::{player::{Player, Resources}, traits::*};

pub struct Smithy;
impl Card for Smithy {
    fn cost(&self) -> u8 {
        return 4;
    }
    
    fn name(&self) -> &'static str {
        return "Smithy";
    }
}
impl Action for Smithy {
    fn effects(&self, mut player: Player) -> Player {
        player.draw_cards(3);
        return player;
    }
}
