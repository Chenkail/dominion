#[path = "../traits/card.rs"] pub mod card;
pub use card as data;
use data::*;

pub struct Copper;

impl Card for Copper {
    fn cost(&self) -> u8 {
        return 0;
    }
    
    fn name(&self) -> &'static str {
        return "Copper";
    }
}

impl Treasure for Copper {
    fn value(&self) -> u8 {
        return 1;
    }
}
pub struct Silver;

impl Card for Silver {
    fn cost(&self) -> u8 {
        return 3;
    }
    
    fn name(&self) -> &'static str {
        return "Silver";
    }
}

impl Treasure for Silver {
    fn value(&self) -> u8 {
        return 2;
    }
}

pub struct Gold;

impl Card for Gold {
    fn cost(&self) -> u8 {
        return 6;
    }
    
    fn name(&self) -> &'static str {
        return "Gold";
    }
}

impl Treasure for Gold {
    fn value(&self) -> u8 {
        return 3;
    }
}
