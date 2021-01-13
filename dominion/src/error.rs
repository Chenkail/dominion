use std::{self, error::Error};
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum DominionError {
    CardTypeMisMatch,
}

#[derive(Debug, Clone)]
pub struct CardTypeMisMatch;

impl Display for CardTypeMisMatch {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "Hi")
    }
}

impl Error for CardTypeMisMatch {
    fn description(&self) -> &str {
        "Hi"
    }
}
