use crate::game::traits::*;

pub struct Nobles;
impl Card for Nobles {
    fn cost(&self) -> i32 {
        return 6;
    }
    
    fn name(&self) -> &'static str {
        return "Nobles";
    }
}
impl Victory for Nobles {
    fn points(&self) -> i32 {
        return 2;
    }
}
// impl Action for Nobles {
//     fn effects(&self) {
        
//     }
// }
