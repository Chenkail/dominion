use super::{traits::Card, cards::base::*};

/// Returns a Card trait object given the name
pub fn card_lookup(name: &str) -> Box<dyn Card> {
    // TODO: Find trait object
    let copper = Box::new(Copper);
    return copper;
}
