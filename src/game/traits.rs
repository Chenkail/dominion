use crate::game::player::Player;

pub trait Card {
    fn cost(&self) -> i32;
    fn name(&self) -> &'static str;

    fn description(&self) -> &'static str {
        return "";
    }
}

pub trait Treasure: Card {
    fn value(&self) -> i32;
}

pub trait Victory: Card {
    fn points(&self) -> i32;
}

pub trait Action: Card {
    fn effects(&self, player: Player) -> Player;
}

pub trait Attack: Action {
    fn attack(&self, target: Player) -> Player;
}

pub trait Reaction: Action {
    fn react(&self, player: Player) -> Player;
}

pub (crate) trait CurseTrait: Card {
    fn points(&self) -> i32 {
        return 1;
    }
}
