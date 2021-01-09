mod game;
use game::cards;
use crate::game::traits::*;

use cards::*;

fn main() {
    let copper = base::Copper;
    println!("{}", copper.cost());

    let silver = base::Silver;
    println!("{}", silver.value());

    let nobles = intrigue::Nobles;
    println!("{}", nobles.points());
}
