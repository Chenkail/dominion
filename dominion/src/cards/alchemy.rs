//! Cards from the Alchemy set

use super::prelude::*;

declare_card!(Potion, "[Wiki link](http://wiki.dominionstrategy.com/index.php/Potion)");
#[typetag::serde]
impl Card for Potion {
    name!("Potion");
    card_cost!(4);
    types!(vec![Treasure]);
    fn potion_value(&self, _: &Player) -> usize { 1 }
}
