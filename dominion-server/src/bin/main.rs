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
                    r = deserialized.try_next() => {
                        if let Some(msg) = r.unwrap() {
                            // println!("GOT: {:?}", msg);
                            match msg {
                                ClientMessage::Ping => {
                                    println!("Got a ping!");
                                    serialized.send(serde_json::to_value(&ServerMessage::PingResponse).unwrap()).await.unwrap();
                                }
                                _ => {
                                    println!("Uh oh!")
                                }
                            }
                        }
                    }
                }
            }
        });
    }
}
