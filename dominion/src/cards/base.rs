//! Base [cards](Card) that get used in every game of Dominion

use super::prelude::*;

card!(Copper, "[Wiki link](http://wiki.dominionstrategy.com/index.php/Copper)");
#[typetag::serde]
impl Card for Copper {
    name!("Copper");
    cost!(0);
    types!(vec![Treasure]);
    treasure_value!(1);
}

card!(Silver, "[Wiki link](http://wiki.dominionstrategy.com/index.php/Silver)");
#[typetag::serde]
impl Card for Silver {
    name!("Silver");
    cost!(3);
    types!(vec![Treasure]);
    treasure_value!(2);
}

card!(Gold, "[Wiki link](http://wiki.dominionstrategy.com/index.php/Gold)");
#[typetag::serde]
impl Card for Gold {
    name!("Gold");
    cost!(6);
    types!(vec![Treasure]);
    treasure_value!(3);
}

card!(Estate, "[Wiki link](http://wiki.dominionstrategy.com/index.php/Estate)");
#[typetag::serde]
impl Card for Estate {
    name!("Estate");
    cost!(2);
    types!(vec![Treasure]);
    victory_points!(1);
}

card!(Duchy, "[Wiki link](http://wiki.dominionstrategy.com/index.php/Duchy)");
#[typetag::serde]
impl Card for Duchy {
    name!("Duchy");
    cost!(5);
    types!(vec![Treasure]);
    victory_points!(3);
}

card!(Province, "[Wiki link](http://wiki.dominionstrategy.com/index.php/Province)");
#[typetag::serde]
impl Card for Province {
    name!("Province");
    cost!(8);
    types!(vec![Treasure]);
    victory_points!(6);
}

card!(BasicCurse, "[Wiki link](http://wiki.dominionstrategy.com/index.php/Curse)");
#[typetag::serde]
impl Card for BasicCurse {
    name!("Curse");
    cost!(0);
    types!(vec![Curse]);
    victory_points!(-1);
}
