//! Cards from the Alchemy set

use super::prelude::*;

/// [Wiki link](http://wiki.dominionstrategy.com/index.php/Potion)
#[derive(Clone, Serialize, Deserialize)]
pub struct Potion;

#[typetag::serde]
impl Card for Potion {
    name!("Potion");
    cost!(4);
    types!(vec![Treasure]);
    fn potion_value(&self, _: &Player) -> usize { 1 }
}
