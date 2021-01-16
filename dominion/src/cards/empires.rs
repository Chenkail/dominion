//! Cards from the Empires set

use super::prelude::*;

card!(Fortune);
impl Card for Fortune {
    name!("Fortune");
    cost!(6, 1);
    types!(vec![TreasureCard]);

    fn treasure_value(&self, player: &mut Player) -> i32 {
        player.coins_remaining
    }

    fn on_gain(&self, player: &mut Player, _: &mut HashMap<Box<dyn Card>, u8>, _: &mut Vec<Player>) {
        for card in player.actions_in_play {
            player.gain(Box::new(Gold));
        }
    }
}
