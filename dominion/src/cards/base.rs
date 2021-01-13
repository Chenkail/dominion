//! Base [cards](Card) that get used in every game of Dominion

use super::prelude::*;

basic_treasure!(Copper, "Copper", 0, 1);
basic_treasure!(Silver, "Silver", 3, 2);
basic_treasure!(Gold, "Gold", 6, 3);
basic_victory!(Estate, "Estate", 2, 1);
basic_victory!(Duchy, "Duchy", 5, 3);
basic_victory!(Province, "Province", 8, 6);
basic_curse!(Curse, "Curse", 0, -1);
