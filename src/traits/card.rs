pub (crate) trait Card {
    fn cost(&self) -> u8;
    fn name(&self) -> &'static str;

    fn description() -> &'static str {
        return "";
    }
}

pub (crate) trait Action {
    
}

pub (crate) trait Treasure {
    fn value(&self) -> u8;
}

pub (crate) trait Victory {
    fn points() -> u8;
}

pub (crate) trait Curse {
    fn points() -> u8;
}

pub (crate) trait Reaction {
    
}
