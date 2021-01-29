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
    fn effects_on_play(&self, player: &mut Player, _: &mut Supply, _: &mut CardDeck, _: &mut PlayerSlice, _: &Callbacks) {
        player.discard.append(&mut player.hand);
        player.draw_cards(7);
    }
}
