//! Base [cards](Card) that get used in every game of Dominion

use super::prelude::*;

basic_treasure!(Copper, "Copper", cost=0, value=1, "[Wiki link](http://wiki.dominionstrategy.com/index.php/Copper)");
basic_treasure!(Silver, "Silver", cost=3, value=2, "[Wiki link](http://wiki.dominionstrategy.com/index.php/Silver)");
basic_treasure!(Gold, "Gold", cost=6, value=3, "[Wiki link](http://wiki.dominionstrategy.com/index.php/Gold)");

basic_victory!(Estate, "Estate", cost=2, points=1, "[Wiki link](http://wiki.dominionstrategy.com/index.php/Estate)");
basic_victory!(Duchy, "Duchy", cost=5, points=3, "[Wiki link](http://wiki.dominionstrategy.com/index.php/Duchy)");
basic_victory!(Province, "Province", cost=8, points=6, "[Wiki link](http://wiki.dominionstrategy.com/index.php/Province)");

basic_curse!(BasicCurse, "BasicCurse", cost=0, points=-1, "[Wiki link](http://wiki.dominionstrategy.com/index.php/Curse)");
