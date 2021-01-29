//! Cards from the Empires set

use super::prelude::*;
use super::base::*;

/// [Wiki link](http://wiki.dominionstrategy.com/index.php/Fortune)
#[derive(Clone, Serialize, Deserialize)]
pub struct Fortune;

#[typetag::serde]
impl Card for Fortune {
    name!("Fortune");
    cost!(8, 8);
    types!(vec![Treasure]);

    fn treasure_value(&self, player: &Player) -> usize {
        if player.state.fortuned {
            0
        } else {
            player.resources.coins_remaining
        }
    }

    fn effects_on_play(&self, player: &mut Player, _: &mut Supply, _: &mut CardDeck, _: &mut PlayerSlice, _: &Callbacks) {
        // This needs to happen AFTER adding the treasure value
        player.state.fortuned = true;
        player.resources.buys += 1;
    }

    fn effects_on_gain(&self, player: &mut Player, supply: &mut Supply, trash: &mut CardDeck, other_players: &mut PlayerSlice, callbacks: &Callbacks) {
        let in_play = player.actions_in_play.clone();
        for card in in_play {
            if card.name() == "Gladiator" {
                let _ = player.gain(Box::new(Gold), supply, trash, other_players, callbacks);
            }
        }
    }
}
