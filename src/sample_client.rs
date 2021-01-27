//! Command line text client for Dominion for demonstrating callbacks

use std::{io, vec};

// use crate::game::player::Player;
use crate::game::Callbacks;

pub fn callbacks() -> Callbacks {
    Callbacks {
        prompt_player_done: Box::new(prompt_done),
        prompt_card_from_hand: Box::new(prompt_card_from_hand),
        prompt_indices_from_hand: Box::new(prompt_indices_from_hand),
    }
}

fn prompt_done() -> bool {
    let mut input = String::new();
    println!("Done? (y)es/(n)o");
    io::stdin().read_line(&mut input).expect("error: unable to read user input");
    !input.to_ascii_lowercase().starts_with('y')
}

fn prompt_card_from_hand() -> usize {
    let mut input = String::new();
    println!("Enter a card index from your hand:");
    io::stdin().read_line(&mut input).expect("error: unable to read user input");
    input.parse::<usize>().unwrap()
}

fn prompt_indices_from_hand() -> Vec<usize> {
    let mut input = String::new();
    let mut output = vec![];
    let prompt = "Enter a card index from your hand, or -1 to stop:";
    println!("{}", prompt);
    io::stdin().read_line(&mut input).expect("error: unable to read user input");
    let mut i = input.parse::<isize>().unwrap();
    while i >= 0 {
        output.push(i as usize);
        println!("{}", prompt);
        io::stdin().read_line(&mut input).expect("error: unable to read user input");
        i = input.parse::<isize>().unwrap();
    }

    output
}
