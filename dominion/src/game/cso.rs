//! "Card-shaped objects"
use super::prelude::*;

pub trait Artifact {

}

pub trait Boon {

}

pub trait Event {
    
}

pub trait Hex {
    
}

pub trait Landmark {
    
}

pub trait Project {
    
}

pub trait State {
    
}

pub trait Way {
    fn effects(&self, _player: Player, _card: Box<dyn Card>, _supply: HashMap<Box<dyn Card>, u8>);
}
