//! Cards from the Adventures set

use super::prelude::*;

placeholder_card!(Champion, "Champion", 6);

card!(Port, "[Wiki link](http://wiki.dominionstrategy.com/index.php/Port)");
#[typetag::serde]
impl Card for Port {
    name!("Port");
    card_cost!(4);
    types!(vec![Action]);
    basic_on_play_effects!(
        cards=1,
        actions=2,
        buys=0,
        coins=0);
    fn effects_on_buy(&self, game: &mut Game, player_index: usize, callbacks: &Callbacks) {
        let _ = game.gain(player_index,Box::new(Port), callbacks);
    }
}
