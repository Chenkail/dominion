//! Cards from the Dark Ages set

use std::cmp::max;

use super::prelude::*;

card!(PoorHouse, "[Wiki link](http://wiki.dominionstrategy.com/index.php/Poor_House)");
#[typetag::serde]
impl Card for PoorHouse {
    name!("Poor House");
    cost!(1);
    types!(vec![Action]);
    fn effects_on_play(&self, game: &mut Game, player_index: usize, callbacks: &Callbacks) {
        let player = &mut game.players[player_index];

        player.resources.temp_coins += 4;
        (callbacks.reveal_hand)(player);
        for card in &player.hand {
            if card.is_treasure() {
                player.resources.temp_coins = max(0, player.resources.temp_coins - 1);
            }
        }
    }
}
