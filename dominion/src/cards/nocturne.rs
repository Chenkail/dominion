//! Cards from the Nocturne set

use super::prelude::*;




// Heirlooms

card!(Pouch, "[Wiki link](http://wiki.dominionstrategy.com/index.php/Pouch)");
#[typetag::serde]
impl Card for Pouch {
    name!("Pouch");
    card_cost!(2);
    types!(vec![Treasure, Heirloom]);
    treasure_value!(1);
    basic_on_play_effects!(0, 0, 1, 0);
}
