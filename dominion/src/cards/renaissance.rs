//! Cards from the Renaissance set

use super::prelude::*;

declare_card!(Scholar, "[Wiki link](http://wiki.dominionstrategy.com/index.php/Scholar)");
#[typetag::serde]
impl Card for Scholar {
    name!("Scholar");
    card_cost!(5);
    types!(vec![Action]);
    fn effects_on_play(&self, game: &mut Game, player_index: usize, _: Box<dyn Callbacks>) {
        let player = &mut game.players[player_index];

        player.discard.append(&mut player.hand);
        player.draw_cards(7);
    }
}
