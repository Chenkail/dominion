//! Cards from the Dark Ages set

use std::cmp::max;

use super::prelude::*;

/// [Wiki link](http://wiki.dominionstrategy.com/index.php/Poor_House)
#[derive(Clone, Serialize, Deserialize)]
pub struct PoorHouse;

#[typetag::serde]
impl Card for PoorHouse {
    name!("Poor House");
    cost!(1);
    types!(vec![Action]);
    fn effects_on_play(&self, player: &mut Player, _: &mut Supply, _: &mut CardDeck, _: &mut PlayerSlice, callbacks: &Callbacks) {
        player.resources.temp_coins += 4;
        (callbacks.reveal_hand)(player);
        for card in &player.hand {
            if card.is_treasure() {
                player.resources.temp_coins = max(0, player.resources.temp_coins - 1);
            }
        }
    }
}
