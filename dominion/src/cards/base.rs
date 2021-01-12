//! Base [cards](Card) that get used in every game of Dominion

use super::prelude::*;

card!(Copper, "Copper", 0, "Treasure");
treasure!(Copper, 1);

card!(Silver, "Silver", 3, "Treasure");
treasure!(Silver, 2);

card!(Gold, "Gold", 6, "Treasure");
treasure!(Gold, 3);

card!(Estate, "Estate", 2, "Victory");
victory!(Estate, 1);

card!(Duchy, "Duchy", 5, "Victory");
victory!(Duchy, 3);

card!(Province, "Province", 8, "Victory");
victory!(Province, 6);

card!(Curse, "Curse", 0, "Curse");
curse!(Curse, -1);
