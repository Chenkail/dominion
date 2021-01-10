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
    /// Effects that the Action card has on the person playing it
    fn effects(&self, player: &mut Player);
}

pub trait Attack: Action {
    /// Effects that the Attack card has
    /// TODO: Figure out how to implement attack cards that hit everyone - boolean flag?
    fn attack(&self, player: &mut Player, target: &mut Player);
}

pub trait Reaction: Action {
    /// Effects that the Reaction card has
    /// TODO: boolean flag for Moat immunity?
    fn react(&self, player: &mut Player);
}

pub (crate) trait CurseTrait: Card {
    fn points(&self) -> i32 {
        return -1;
    }
}
