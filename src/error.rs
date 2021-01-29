//! Dominion error types

use crate::game::Card;
use crate::types::CardType;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum DominionError {
    #[error("Card doesn't have expected type! Expected: {:?}", .expected)]
    CardTypeMisMatch { expected: CardType },
    #[error("Pile is empty: {:?}", .card)]
    EmptyPile { card: Box<dyn Card> },
}
