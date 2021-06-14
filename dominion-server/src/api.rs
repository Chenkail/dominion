use dominion::game::prelude::*;

use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub enum ClientMessage {
    Ping,
    StartGame { supply_list: CardList },
    PlayCard { card: Box<dyn Card> },
    ChatMessage { message: String },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub enum ServerMessage {
    PingResponse,
    StartingGame { state: PartialGame },
    CurrentState { state: PartialGame },
    GameAlreadyStarted,
    ChatMessage { author: usize, message: String },
    // ChatMessage { author: Uuid, message: String },
    NotEnoughPlayers,
}
