//! Cards from the Guilds set

use super::prelude::*;
use super::base::*;

card!(Soothsayer, "[Wiki link](http://wiki.dominionstrategy.com/index.php/Soothsayer)");
#[typetag::serde]
impl Card for Soothsayer {
    name!("Soothsayer");
    cost!(5);
    types!(vec![Action, Attack]);
    fn effects_on_play(&self, game: &mut Game, player_index: usize, callbacks: &Callbacks) {
        // If there are no more Golds, we don't care so we move on
        let _ = game.gain(player_index, Box::new(Gold), callbacks);
    }

    fn attack_targets(&self) -> Option<AttackTargetType> { Some(EveryoneElse) }

    fn attack_effects(&self, game: &mut Game, player_index: usize, callbacks: &Callbacks) {
        // Try to give them a curse
        if game.gain(player_index, Box::new(BasicCurse), callbacks).is_ok() {
            // Only draw a card if they gained a curse
            let player = &mut game.players[player_index];
            player.draw_cards(1);
        }
    }
}
