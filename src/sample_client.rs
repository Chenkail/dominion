use std::io;

use crate::game::player::Player;
use crate::game::callbacks::*;

pub fn text_client() -> Callbacks {
    Callbacks {
        prompt_player_done: Box::new(prompt_done),
        prompt_card_from_hand: Box::new(prompt_card_from_hand),
    }
}

fn prompt_done() -> bool {
    let mut input = String::new();
    println!("Done? (y)es/(n)o");
    io::stdin().read_line(&mut input).expect("error: unable to read user input");
    !input.to_ascii_lowercase().starts_with('y')
}

fn prompt_card_from_hand(_p: &Player) -> i32 {
    0
}
