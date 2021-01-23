use thiserror::Error;

#[derive(Error, Debug)]
pub enum DominionError {
    #[error("Card doesn't have expected type! Expected: {}", .expected)]
    CardTypeMisMatch { expected: String },
}
