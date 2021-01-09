mod items;
use items::cards;
use items::traits::card::*;

use cards::*;

fn main() {
    let copper = base::Copper;
    println!("{}", copper.cost());

    let silver = base::Silver;
    println!("{}", silver.value());

    let nobles = intrigue::Nobles;
    println!("{}", nobles.points());
}
