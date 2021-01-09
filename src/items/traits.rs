pub (crate) trait Card {
    fn cost(&self) -> u8;
    fn name(&self) -> &'static str;

    fn description() -> &'static str {
        return "";
    }
}

pub (crate) trait Action: Card {
    
}

pub (crate) trait Treasure: Card {
    fn value(&self) -> u8;
}

pub (crate) trait Victory: Card {
    fn points(&self) -> u8;
}

pub (crate) trait CurseTrait: Card {
    fn points(&self) -> u8 {
        return 1;
    }
}

pub (crate) trait Reaction: Action {
    
}
