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

    fn effects_on_play(&self, game: &mut Game, current_player_index: usize, _: &Callbacks) {
        let player = &mut game.players[current_player_index];

        // This needs to happen AFTER adding the treasure value
        player.state.fortuned = true;
        player.resources.buys += 1;
    }

    fn effects_on_gain(&self, game: &mut Game, current_player_index: usize, callbacks: &Callbacks) {
        let player = &game.players[current_player_index];

        let in_play = player.in_play.clone();
        for card in in_play {
            if card.name() == "Gladiator" {
                let _ = game.gain(current_player_index, Box::new(Gold), callbacks);
            }
        }
    }
}
