#[derive(Debug)]
pub enum DominionError {
    CardTypeMisMatch { expected: String },
}

