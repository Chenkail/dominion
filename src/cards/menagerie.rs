//! Cards from the Menagerie set

use super::prelude::*;

/// [Wiki link](http://wiki.dominionstrategy.com/index.php/Horse)
#[derive(Clone, Serialize, Deserialize)]
pub struct Horse;

#[typetag::serde]
impl Card for Horse {
    name!("Horse");
    cost!(3);
    types!(vec![Action]);
    fn effects_on_play(&self, player: &mut Player, _: &mut Supply, _: &mut CardDeck, _: &mut PlayerSlice, _: &Callbacks) {
        player.draw_cards(2);
        player.add_actions(1);

        if player.in_play.back().unwrap().name() == "Horse" {
            player.in_play.pop_back().unwrap();
            // TODO: Increment Horse pile count
        }
    }
}
