pub use serde::{Serialize, Deserialize};

use dominion::game::prelude::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Message {
    Ping,
    GetHand,
    StartGame,
    PlayCard { card: Box<dyn Card> },
}
