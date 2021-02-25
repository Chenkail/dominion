//! Command line text client for Dominion for demonstrating callbacks

use std::{io, vec};
use itertools::Itertools;

use crate::{game::Card, types::Supply};

// use crate::game::player::Player;
use crate::game::{Callbacks, Player};

pub fn callbacks() -> Callbacks {
    Callbacks {
        prompt_player_done: Box::new(prompt_done),
        prompt_card_from_hand: Box::new(prompt_card_from_hand),
        prompt_indices_from_hand: Box::new(prompt_indices_from_hand),
        prompt_indices_from_hand_u: Box::new(prompt_indices_from_hand_u),
        reveal_hand: Box::new(reveal_hand),
        reveal_top_discard_pile: Box::new(reveal_top_discard_pile),
        reveal_top_draw_pile: Box::new(reveal_top_draw_pile),
        get_player_consent: Box::new(get_player_consent),
        choose_card_from_supply: Box::new(choose_card_from_supply),
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


fn prompt_indices_from_hand_u(num_cards: usize) -> Vec<usize> {
    let mut input = String::new();
    let mut output = vec![];
    let prompt = "Enter a card index from your hand, or -1 to stop:";
    println!("{}", prompt);
    io::stdin().read_line(&mut input).expect("error: unable to read user input");
    let mut i = input.parse::<isize>().unwrap();
    let mut j = 0;
    while i >= 0 && j < num_cards {
        output.push(i as usize);
        println!("{}", prompt);
        io::stdin().read_line(&mut input).expect("error: unable to read user input");
        i = input.parse::<isize>().unwrap();
        j += 1;
    }
    output
}

fn reveal_hand(player: &Player) {
    player.print_hand()
}

fn reveal_top_discard_pile(player: &Player, n: usize) {
    for i in 0..n {
        println!("{}", player.discard.get(i).unwrap().name())
    }
}

fn reveal_top_draw_pile(player: &Player, n: usize) {
    for i in 0..n {
        println!("{}: {}",n , player.deck.get(i).unwrap().name())
    }
}

fn get_player_consent(_player: &mut Player) -> bool {
    let mut input = String::new();
    println!("(y)es/(n)o");
    io::stdin().read_line(&mut input).expect("error: unable to read user input");

    input.starts_with('y')
}

fn choose_card_from_supply(supply: &Supply) -> Box<dyn Card> {
    let mut alphabetized = supply.keys().collect_vec();
    alphabetized.sort_unstable();
    println!("Cards:");
    println!("{:?}", alphabetized);

    println!("Enter index to select:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("error: unable to read user input");
    let i = input.parse::<usize>().unwrap();
    let card = *alphabetized.get(i).expect("Index out of bounds");

    card.clone()
}
