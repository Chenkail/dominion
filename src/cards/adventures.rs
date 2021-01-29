//! Cards from the Adventures set

use super::prelude::*;

/// [Wiki link](http://wiki.dominionstrategy.com/index.php/Port)
#[derive(Clone, Serialize, Deserialize)]
pub struct Port;

#[typetag::serde]
impl Card for Port {
    name!("Port");
    cost!(4);
    types!(vec![Action]);
    basic_on_play_effects!(1, 2, 0, 0);
    fn effects_on_buy(&self, player: &mut Player, supply: &mut Supply, trash: &mut CardDeck, other_players: &mut PlayerSlice, callbacks: &Callbacks) {
        player.gain(Box::new(Port), supply, trash, other_players, callbacks);
    }
}
