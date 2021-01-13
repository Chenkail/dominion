//! Base [cards](Card) that get used in every game of Dominion

use super::prelude::*;

pub struct Copper;
impl Card for Copper {
    name!("Copper");
    cost!(0);
    types!(vec!["Treasure"]);
    treasure_value!(1);
}

card!(Silver, "Silver", 3);

card!(Gold, "Gold", 6);

card!(Estate, "Estate", 2);

card!(Duchy, "Duchy", 5);

card!(Province, "Province", 8);

card!(Curse, "Curse", 0);
