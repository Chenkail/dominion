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
    fn effects_on_play(&self, player: &mut Player, supply: &mut Supply, trash: &mut CardDeck, other_players: &mut PlayerSlice, callbacks: &Callbacks) {
        // If there are no more Golds, we don't care so we move on
        let _ = player.gain(Box::new(Gold), supply, trash, other_players, callbacks);

        for p in other_players {
            // FIXME: This doesn't actually pass in other_players but I'm
            // not sure how we can make that work. There are (as of jan 2021)
            // no reaction cards that both care about being given a curse
            // AND have effects that impact other players
            let r = p.gain(Box::new(BasicCurse), supply, trash, &mut Vec::new(), callbacks);
            if r.is_ok() {
                p.draw_cards(1);
            }
        }
    }
}
