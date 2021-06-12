use std::sync::{Arc, Mutex};

use tokio::{io::{AsyncBufReadExt, AsyncWriteExt, BufReader}, net::{TcpListener, TcpStream}, sync::broadcast};

use tokio_serde::formats::*;
use tokio_util::codec::{FramedRead, FramedWrite, LengthDelimitedCodec};


use dominion::game::prelude::*;
use dominion::error::DominionError::*;


use futures::prelude::*;
use serde_json::{json, Value};
use tokio_serde::formats::*;

type Recipients = Vec<usize>;


#[tokio::main]
pub async fn main() {
    // Bind a server socket
    let listener = TcpListener::bind("localhost:8080").await.unwrap();
    let (tx, _rx) = broadcast::channel::<(String, Recipients)>(10);

    let data = Arc::new(Mutex::new(Game::new()));
    let mut player_count = 0;

    loop {
        let (socket, _addr) = listener.accept().await.unwrap();

        let tx = tx.clone();
        let mut rx = tx.subscribe();

        if player_count > 5 {
            println!("Too many players already!");
            continue;
        }

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

        // Delimit frames using a length header
        let length_delimited = FramedRead::new(socket, LengthDelimitedCodec::new());

        // Deserialize frames
        let mut deserialized = tokio_serde::SymmetricallyFramed::new(
            length_delimited,
            SymmetricalJson::<Value>::default(),
        );

        let length_delimited = FramedWrite::new(socket2, LengthDelimitedCodec::new());
        let mut serialized =
            tokio_serde::SymmetricallyFramed::new(length_delimited, SymmetricalJson::default());

        // Spawn a task that prints all received messages to STDOUT
        tokio::spawn(async move {
            while let Some(msg) = deserialized.try_next().await.unwrap() {
                println!("GOT: {:?}", msg);

                serialized
                    .send(json!({
                        "name": "John Doe",
                        "age": 43,
                        "phones": [
                            "+44 1234567",
                            "+44 2345678"
                        ]
                    }))
                    .await
                    .unwrap()
            }
        });
    }
}
