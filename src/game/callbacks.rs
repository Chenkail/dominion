use crate::game::player::Player;

pub type FnToBool = Box<dyn Fn() -> bool>;
pub type FnPlayerToi32 = Box<dyn Fn(&Player) -> i32>;

pub struct Callbacks {
    pub prompt_player_done: FnToBool,
    pub prompt_card_from_hand: FnPlayerToi32,
}
