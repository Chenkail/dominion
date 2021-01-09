use crate::items::traits::*;

pub struct Nobles;
impl Card for Nobles {
    fn cost(&self) -> u8 {
        return 6;
    }
    
    fn name(&self) -> &'static str {
        return "Nobles";
    }
}
impl Victory for Nobles {
    fn points(&self) -> u8 {
        return 2;
    }
}
impl Action for Nobles {
    
}
