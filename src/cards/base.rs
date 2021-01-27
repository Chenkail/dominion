//! Base [cards](Card) that get used in every game of Dominion

use super::prelude::*;

/// [Wiki link](http://wiki.dominionstrategy.com/index.php/Copper)
#[derive(Clone, Serialize, Deserialize)]
pub struct Copper;

#[typetag::serde]
impl Card for Copper {
    name!("Copper");
    cost!(0);
    types!(vec![Treasure]);
    treasure_value!(1);
}

/// [Wiki link](http://wiki.dominionstrategy.com/index.php/Silver)
#[derive(Clone, Serialize, Deserialize)]
pub struct Silver;

#[typetag::serde]
impl Card for Silver {
    name!("Silver");
    cost!(3);
    types!(vec![Treasure]);
    treasure_value!(2);
}

/// [Wiki link](http://wiki.dominionstrategy.com/index.php/Gold)
#[derive(Clone, Serialize, Deserialize)]
pub struct Gold;

#[typetag::serde]
impl Card for Gold {
    name!("Gold");
    cost!(6);
    types!(vec![Treasure]);
    treasure_value!(3);
}

/// [Wiki link](http://wiki.dominionstrategy.com/index.php/Estate)
#[derive(Clone, Serialize, Deserialize)]
pub struct Estate;

#[typetag::serde]
impl Card for Estate {
    name!("Estate");
    cost!(2);
    types!(vec![Treasure]);
    victory_points!(1);
}

/// [Wiki link](http://wiki.dominionstrategy.com/index.php/Duchy)
#[derive(Clone, Serialize, Deserialize)]
pub struct Duchy;

#[typetag::serde]
impl Card for Duchy {
    name!("Duchy");
    cost!(5);
    types!(vec![Treasure]);
    victory_points!(3);
}

/// [Wiki link](http://wiki.dominionstrategy.com/index.php/Province)
#[derive(Clone, Serialize, Deserialize)]
pub struct Province;

#[typetag::serde]
impl Card for Province {
    name!("Province");
    cost!(8);
    types!(vec![Treasure]);
    victory_points!(6);
}

/// [Wiki link](http://wiki.dominionstrategy.com/index.php/Curse)
#[derive(Clone, Serialize, Deserialize)]
pub struct BasicCurse;

#[typetag::serde]
impl Card for BasicCurse {
    name!("Curse");
    cost!(0);
    types!(vec![Curse]);
    victory_points!(-1);
}
