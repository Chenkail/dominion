//! "Card-shaped objects"
use crate::game::{player::Player, card::Card};
use crate::types::*;

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
    fn effects(&self, _player: Player, _card: Box<dyn Card>, _supply: Supply);
}
