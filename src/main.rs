mod cards;

use cards::base;
use cards::base::data::{Card, Treasure};

fn main() {
    let cop = base::Copper;
    println!("{}", cop.cost());

    let sil = base::Silver;
    println!("{}", sil.value());
}
