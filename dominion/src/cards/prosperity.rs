//! Cards from the Prosperity set

use super::prelude::*;

/// [Wiki link](http://wiki.dominionstrategy.com/index.php/Colony)
#[derive(Clone, Serialize, Deserialize)]
pub struct Colony;

#[typetag::serde]
impl Card for Colony {
    name!("Colony");
    cost!(11);
    types!(vec![Victory]);
    victory_points!(10);
}

/// [Wiki link](http://wiki.dominionstrategy.com/index.php/Platinum)
#[derive(Clone, Serialize, Deserialize)]
pub struct Platinum;

#[typetag::serde]
impl Card for Platinum {
    name!("Platinum");
    cost!(9);
    types!(vec![Treasure]);
    treasure_value!(5);
}
