//! Utility macros

/// Easily create a vector of Boxed Card trait objects
///
/// For example:
/// ```
/// use dominion::game::prelude::*;
/// use dominion::cards::base::*;
///
/// let cards = card_vec![Copper, Silver, Gold];
/// ```
#[macro_export]
macro_rules! card_vec {
    ( $( $card:expr ),* ) => {
        {
            let mut v: CardList = Vec::new();
            $(v.push(Box::new($card));)*

            v
        }
    };
}

#[macro_export]
macro_rules! supply_add {
    ($supply:expr, $card:expr, $quantity:expr) => {
        {
            $supply.insert($card.name().to_string(), SupplyEntry { card: Box::new($card), count: $quantity })
        }
    };
}
