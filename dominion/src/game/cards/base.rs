//! Base [cards](Card) that get used in every game of Dominion

use super::prelude::*;

card!(Copper, "Copper", 0);
treasure!(Copper, 1);

card!(Silver, "Silver", 3);
treasure!(Silver, 2);

card!(Gold, "Gold", 6);
treasure!(Gold, 3);

card!(Estate, "Estate", 2);
victory!(Estate, 1);

card!(Duchy, "Duchy", 5);
victory!(Duchy, 3);

card!(Province, "Province", 8);
victory!(Province, 6);

card!(Curse, "Curse", 0);
curse!(Curse, -1);
