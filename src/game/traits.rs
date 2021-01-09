pub trait Card {
    fn cost(&self) -> u8;
    fn name(&self) -> &'static str;

    fn description(&self) -> &'static str {
        return "";
    }
}

pub trait Treasure: Card {
    fn value(&self) -> u8;
}

pub trait Victory: Card {
    fn points(&self) -> u8;
}

pub trait Action: Card {
    fn effects(&self);
}

pub trait Attack: Action {
    fn attack(&self);
}

pub trait Reaction: Action {
    fn react(&self);
}

pub (crate) trait CurseTrait: Card {
    fn points(&self) -> u8 {
        return 1;
    }
}
