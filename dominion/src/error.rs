//! Dominion error types

use crate::game::Card;
use crate::types::CardType;

use thiserror::Error;

pub type DominionResult = Result<(), DominionError>;

#[derive(Debug, Error)]
pub enum DominionError {
    #[error("Card doesn't have expected type! Expected: {expected:?}")]
    CardTypeMisMatch { expected: CardType },
    #[error("Pile is empty: {card:?}")]
    EmptyPile { card: Box<dyn Card> },
    #[error("Not enough money to buy that card!")]
    InsufficientFunds,
    #[error("Not enough players to start!")]
    NotEnoughPlayers,
}
