//! Cards from the Guilds set

use super::prelude::*;
use super::base::*;

/// [Wiki link](http://wiki.dominionstrategy.com/index.php/Soothsayer)
#[derive(Clone, Serialize, Deserialize)]
pub struct Soothsayer;

#[typetag::serde]
impl Card for Soothsayer {
    name!("Soothsayer");
    cost!(5);
    types!(vec![Action, Attack]);
    fn effects_on_play(&self, game: &mut Game, player_index: usize, callbacks: &Callbacks) {
        // If there are no more Golds, we don't care so we move on
        let _ = game.gain(player_index, Box::new(Gold), callbacks);

        for i in 0..game.players.len() {
            if i != player_index {
                let index = (i+game.players.len()) % game.players.len();
                let r = game.gain(index, Box::new(BasicCurse), callbacks);
                if r.is_ok() {
                    let player = &mut game.players[index];
                    player.draw_cards(1);
                }
            }
        }
    }
}
