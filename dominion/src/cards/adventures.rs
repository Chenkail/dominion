//! Cards from the Adventures set

use super::prelude::*;

placeholder_card!(Champion, "Champion", 6);

/// [Wiki link](http://wiki.dominionstrategy.com/index.php/Port)
#[derive(Clone, Serialize, Deserialize)]
pub struct Port;

#[typetag::serde]
impl Card for Port {
    name!("Port");
    cost!(4);
    types!(vec![Action]);
    basic_on_play_effects!(1, 2, 0, 0);
    fn effects_on_buy(&self, game: &mut Game, player_index: usize, callbacks: &Callbacks) {
        let _ = game.gain(player_index,Box::new(Port), callbacks);
    }
}
