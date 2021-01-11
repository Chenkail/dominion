use std::collections::VecDeque;

use crate::game::{traits::Card, cards::base::*};

/// Returns a Card trait object given the name
pub fn card_lookup(name: &str) -> Box<dyn Card> {
    // TODO: Find trait object
    let copper = Box::new(Copper);
    return copper;
}

/// Shuffles a VecDeque of cards (or rather a VecDeque\<Box\<dyn Card\>\>)
pub fn shuffle(pile: &mut VecDeque<Box<dyn Card>>) {

}
