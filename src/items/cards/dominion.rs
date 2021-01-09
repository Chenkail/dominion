use crate::items::traits::*;

pub struct Smithy;
impl Card for Smithy {
    fn cost(&self) -> u8 {
        return 4;
    }
    
    fn name(&self) -> &'static str {
        return "Smithy";
    }
}
impl Action for Smithy {
    fn effects(&self) {
        
    }
}
