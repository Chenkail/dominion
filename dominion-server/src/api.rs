pub use serde::{Serialize, Deserialize};

use dominion::game::prelude::*;

#[derive(Clone, Serialize, Deserialize)]
enum Message {
    Ping,
    GetHand,
    StartGame,
    PlayCard { card: Box<dyn Card> },
}
