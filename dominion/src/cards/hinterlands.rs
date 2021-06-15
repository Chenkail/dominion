//! Cards from the Hinterlands set

use super::prelude::*;

card!(SilkRoad, "[Wiki link](http://wiki.dominionstrategy.com/index.php/Silk_Road)");
#[typetag::serde]
impl Card for SilkRoad {
    name!("Silk Road");
    cost!(4);
    types!(vec![Victory]);
    fn victory_points(&self, player: &Player) -> isize {
        let mut victory_cards = 0;
        for card in player.card_iter() {
            if card.is_victory() {
                victory_cards += 1;
            }
        }

        victory_cards / 4
    }
}
