use crate::game::traits::*;

pub struct Copper;
impl Card for Copper {
    fn cost(&self) -> i32 {
        return 0;
    }
    
    fn name(&self) -> &'static str {
        return "Copper";
    }
}
impl Treasure for Copper {
    fn value(&self) -> i32 {
        return 1;
    }
}

pub struct Silver;
impl Card for Silver {
    fn cost(&self) -> i32 {
        return 3;
    }
    
    fn name(&self) -> &'static str {
        return "Silver";
    }
}
impl Treasure for Silver {
    fn value(&self) -> i32 {
        return 2;
    }
}

pub struct Gold;
impl Card for Gold {
    fn cost(&self) -> i32 {
        return 6;
    }
    
    fn name(&self) -> &'static str {
        return "Gold";
    }
}
impl Treasure for Gold {
    fn value(&self) -> i32 {
        return 3;
    }
}

pub struct Estate;
impl Card for Estate {
    fn cost(&self) -> i32 {
        return 2;
    }
    
    fn name(&self) -> &'static str {
        return "Estate";
    }
}
impl Victory for Estate {
    fn points(&self) -> i32 {
        return 1;
    }
}

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
