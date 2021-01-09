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

    let player1 = Player::new();
    for card in player1.deck {
        // let ca = *card.;
    }
}
