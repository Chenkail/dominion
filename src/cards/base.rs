//! Base [cards](Card) that get used in every game of Dominion

use super::prelude::*;

/// [Wiki link](http://wiki.dominionstrategy.com/index.php/Copper)
#[derive(Clone)]
#[derive(Serialize, Deserialize)]
pub struct Copper;

#[typetag::serde]
impl Card for Copper {
    name!("Copper");
    cost!(0);
    types!(vec![Treasure]);
}

basic_treasure!(Silver, "Silver", 3, 2);
basic_treasure!(Gold, "Gold", 6, 3);
basic_victory!(Estate, "Estate", 2, 1);
basic_victory!(Duchy, "Duchy", 5, 3);
basic_victory!(Province, "Province", 8, 6);
basic_curse!(BasicCurse, "Curse", 0, -1);
