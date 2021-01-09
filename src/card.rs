struct Card {
    cost: u8,
    name: &'static str,
    text: &'static str
}

trait Action {
    
}

trait Treasure {
    fn value(&self) -> u8;
}

trait Victory {
    fn points(&self) -> u8;
}

trait Reaction {
    
}
