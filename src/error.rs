//! Dominion error types

use crate::types::CardType;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum DominionError {
    #[error("Card doesn't have expected type! Expected: {:?}", .expected)]
    CardTypeMisMatch { expected: CardType },
}
