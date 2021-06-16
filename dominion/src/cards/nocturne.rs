//! Cards from the Nocturne set

use super::prelude::*;




// Heirlooms

declare_card!(Pouch, "[Wiki link](http://wiki.dominionstrategy.com/index.php/Pouch)");
#[typetag::serde]
impl Card for Pouch {
    name!("Pouch");
    card_cost!(2);
    types!(vec![Treasure, Heirloom]);
    treasure_value!(1);
    basic_on_play_effects!(
        cards=0,
        actions=0,
        buys=1,
        coins=0);
}
