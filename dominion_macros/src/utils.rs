//! Utility macros

/// Easily create a vector of Boxed Card trait objects
///
/// ```
/// use dominion_macros::card_vec;
///
/// let cards = card_vec![Copper, Silver, Gold];
/// ```
#[macro_export]
macro_rules! card_vec {
    ( $( $card:expr ),* ) => {
        let mut v: Vec<Box<dyn Card>> = Vec::new();
        $(v.push(Box::new($card));)*

        // let v: Vec<Box<dyn Card>> = vec![
        //     $($card, )*
        // ];

        v
    }
}

        