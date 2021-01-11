/// Base cards that get used in every game of Dominion

use crate::game::traits::*;
use dominion_macros::*;

card!(Copper, "Copper", 0);
treasure!(Copper, 1);

card!(Silver, "Silver", 3);
treasure!(Silver, 2);

card!(Gold, "Gold", 6);
treasure!(Gold, 3);

card!(Estate, "Estate", 2);
victory!(Estate, 1);

pub struct Duchy;
impl Card for Duchy {
    fn cost(&self) -> i32 {
        return 5;
    }
    
    fn name(&self) -> &'static str {
        return "Duchy";
    }
}
impl Victory for Duchy {
    fn points(&self) -> i32 {
        return 3;
    }
}

pub struct Province;
impl Card for Province {
    fn cost(&self) -> i32 {
        return 8;
    }
    
    fn name(&self) -> &'static str {
        return "Province";
    }
}
impl Victory for Province {
    fn points(&self) -> i32 {
        return 6;
    }
}

pub struct Curse;
impl Card for Curse {
    fn cost(&self) -> i32 {
        return 0;
    }
    
    fn name(&self) -> &'static str {
        return "Curse";
    }
}
impl CurseTrait for Curse {
    fn points(&self) -> i32 {
        return -1;
    }
}
