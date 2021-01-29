//! Cards from the Renaissance set

use super::prelude::*;

/// [Wiki link](http://wiki.dominionstrategy.com/index.php/Scholar)
#[derive(Clone, Serialize, Deserialize)]
pub struct Scholar;

#[typetag::serde]
impl Card for Scholar {
    name!("Scholar");
    cost!(5);
    types!(vec![Action]);
    fn effects_on_play(&self, game: &mut Game, current_player_index: usize, _: &Callbacks) {
        let player = &mut game.players[current_player_index];

        player.discard.append(&mut player.hand);
        player.draw_cards(7);
    }
}
