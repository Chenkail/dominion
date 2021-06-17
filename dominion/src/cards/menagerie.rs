//! Cards from the Menagerie set

use super::prelude::*;

declare_card!(Horse, "[Wiki link](http://wiki.dominionstrategy.com/index.php/Horse)");
#[typetag::serde]
impl Card for Horse {
    name!("Horse");
    card_cost!(3);
    types!(vec![Action]);
    fn effects_on_play(&self, game: &mut Game, player_index: usize, _: Box<dyn Callbacks>) {
        let player = &mut game.players[player_index];

        player.draw_cards(2);
        player.add_actions(1);

        if player.in_play.back().unwrap().name().to_lowercase() == "horse" {
            player.in_play.pop_back().unwrap();
            // TODO: Increment Horse pile count
        }
    }
}
