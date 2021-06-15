//! Base [cards](Card) that get used in every game of Dominion

use super::prelude::*;

basic_treasure!(Copper, "Copper", 0, 1, "[Wiki link](http://wiki.dominionstrategy.com/index.php/Copper)");
basic_treasure!(Silver, "Silver", 3, 2, "[Wiki link](http://wiki.dominionstrategy.com/index.php/Silver)");
basic_treasure!(Gold, "Gold", 6, 3, "[Wiki link](http://wiki.dominionstrategy.com/index.php/Gold)");

basic_victory!(Estate, "Estate", 2, 1, "[Wiki link](http://wiki.dominionstrategy.com/index.php/Estate)");
basic_victory!(Duchy, "Duchy", 5, 3, "[Wiki link](http://wiki.dominionstrategy.com/index.php/Duchy)");
basic_victory!(Province, "Province", 8, 6, "[Wiki link](http://wiki.dominionstrategy.com/index.php/Province)");

basic_curse!(BasicCurse, "BasicCurse", 0, -1, "[Wiki link](http://wiki.dominionstrategy.com/index.php/Curse)");
