use dominion::game::prelude::*;

use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ClientMessage {
    Ping,
    GetHand,
    StartGame,
    PlayCard { card: Box<dyn Card> },
    ChatMessage { message: String },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ServerMessage {
    PingResponse,
    StartingGame,
    GameAlreadyStarted,
    ChatMessage { author: usize, message: String },
    // ChatMessage { author: Uuid, message: String },
    NotEnoughPlayers,
}
