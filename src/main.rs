mod game;
use game::{cards, player, traits::*};

use cards::*;
use player::Player;

fn main() {
    let copper = base::Copper;
    println!("{}", copper.cost());

    let silver = base::Silver;
    println!("{}", silver.value());

    let nobles = intrigue::Nobles;
    println!("{}", nobles.points());
    
    println!();
    let mut player = Player::new();
    for card in &player.hand {
        println!("{}", &*card.name());
    }

    println!();
    for card in &player.deck {
        println!("{}", &*card.name());
    }

    player.cleanup();

    println!();
    for card in &player.discard {
        println!("{}", &*card.name());
    }
}
