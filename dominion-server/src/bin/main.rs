use dominion_server::api::{ClientMessage, ServerMessage};

use std::sync::{Arc, Mutex};

use tokio::{net::{TcpListener, TcpStream}, sync::broadcast};

use tokio_serde::formats::*;
use tokio_util::codec::{FramedRead, FramedWrite, LengthDelimitedCodec};

use futures::prelude::*;
use serde_json::Value;

use dominion::game::prelude::*;
use dominion::error::DominionError::*;


type Recipients = Vec<usize>;
fn single_recipient(player_number: usize) -> Recipients {
    vec![player_number]
}

fn everyone_but(player_count: usize, player_number: usize) -> Recipients {
    let mut v = (0..player_count).collect::<Vec<usize>>();
    v.remove(player_number);

    v
}

fn everyone(player_count: usize) -> Recipients {
    (0..player_count).collect::<Vec<usize>>()
}

#[tokio::main]
pub async fn main() {
    // Bind a server socket
    let listener = TcpListener::bind("localhost:8080").await.unwrap();
    let (tx, _rx) = broadcast::channel::<(Value, Recipients)>(10);

    let data = Arc::new(Mutex::new(Game::new()));
    let mut player_count = 0;

    loop {
        let (socket, _addr) = listener.accept().await.unwrap();

        if player_count > 5 {
            println!("Too many players already! Ignoring new connection");
            continue;
        }

        let tx = tx.clone();
        let mut rx = tx.subscribe();

        let player_number = player_count;
        let player = Player::new_with_default_deck(player_number);
        println!("Player #{} joined with UUID: {}", &player.number, &player.uuid);
        player_count += 1;

        let new_data = Arc::clone(&data);
        {
            let mut game = new_data.lock().unwrap();
            game.add_player(player);
        }

        let socket = socket.into_std().unwrap();
        let socket2 = socket.try_clone().unwrap();
        let socket = TcpStream::from_std(socket).unwrap();
        let socket2 = TcpStream::from_std(socket2).unwrap();

        let length_delimited = FramedRead::new(socket, LengthDelimitedCodec::new());
        let mut deserialized = tokio_serde::SymmetricallyFramed::new(
            length_delimited,
            SymmetricalJson::<ClientMessage>::default(),
        );

        let length_delimited = FramedWrite::new(socket2, LengthDelimitedCodec::new());
        let mut serialized =
            tokio_serde::SymmetricallyFramed::new(length_delimited, SymmetricalJson::default());

        // Spawn a task to handle incoming messages
        tokio::spawn(async move {
            loop {
                tokio::select! {
                    result = deserialized.try_next() => {
                        if let Some(msg) = result.unwrap() {
                            // println!("GOT: {:?}", msg);
                            match msg {
                                ClientMessage::Ping => {
                                    println!("Got a ping from player {}!", player_number);
                                    serialized.send(serde_json::to_value(&ServerMessage::PingResponse).unwrap()).await.unwrap();
                                }
                                ClientMessage::ChatMessage{ message } => {
                                    let game = new_data.lock().unwrap();
                                    let player_count = game.players.len();
                                    // let sender = &game.players[player_number];
                                    // let author = sender.uuid;
                                    let message = serde_json::to_value(&ServerMessage::ChatMessage{ author: player_number, message }).unwrap();
                                    let recipients = everyone_but(player_count, player_number);
                                    tx.send((message, recipients)).unwrap();
                                }
                                ClientMessage::StartGame => {
                                    let mut game = new_data.lock().unwrap();
                                    if game.started {
                                        let recipients = single_recipient(player_number);
                                        let message = serde_json::to_value(&ServerMessage::GameAlreadyStarted).unwrap();
                                        tx.send((message, recipients)).unwrap();
                                        continue;
                                    }

                                    let supply_list = Game::default_supply_list();

                                    match game.generate_supply(supply_list) {
                                        Ok(()) => {
                                            game.started = true;
                                            let recipients = single_recipient(player_number);
                                            let message = serde_json::to_value(&ServerMessage::StartingGame).unwrap();
                                            tx.send((message, recipients)).unwrap();
                                        }
                                        Err(NotEnoughPlayers) => {
                                            let recipients = single_recipient(player_number);
                                            let message = serde_json::to_value(&ServerMessage::NotEnoughPlayers).unwrap();
                                            tx.send((message, recipients)).unwrap();
                                        }
                                        _ => panic!("Unknown error while starting!")
                                    }
                                }
                                _ => {
                                    println!("Uh oh!")
                                }
                            }
                        }
                    }

                    result = rx.recv() => {
                        let (value, recipients) = result.unwrap();

                        if recipients.contains(&player_number) {
                            serialized.send(value).await.unwrap();
                        }
                    }
                }
            }
        });
    }
}
