//! Cards from the Seaside set

use super::prelude::*;

/// [Wiki link](http://wiki.dominionstrategy.com/index.php/Bazaar)
#[derive(Clone, Serialize, Deserialize)]
pub struct Bazaar;

#[typetag::serde]
impl Card for Bazaar {
    name!("Bazaar");
    cost!(5);
    types!(vec![Action]);
    basic_on_play_effects!(1, 2, 0, 1);
}
