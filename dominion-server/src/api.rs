use dominion::game::prelude::*;

use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ClientMessage {
    Ping,
    GetHand,
    StartGame,
    PlayCard { card: Box<dyn Card> },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ServerMessage {
    PingResponse,
    StartingGame,
    GameAlreadyStarted,
    NotEnoughPlayers,
}
