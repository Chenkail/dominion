use std::sync::{Arc, Mutex};

use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
    sync::broadcast
};

use dominion::game::prelude::*;
use dominion::error::DominionError::*;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("localhost:8080").await.unwrap();
    let (tx, _rx) = broadcast::channel(10);

    let data = Arc::new(Mutex::new(Game::new()));
    let mut player_number = 0;

    loop {
        let (mut socket, addr) = listener.accept().await.unwrap();

        let tx = tx.clone();
        let mut rx = tx.subscribe();


        if player_number > 5 {
            println!("Too many players already!");
            continue;
        }

        let player = Player::new_with_default_deck(player_number);
        println!("Player #{} joined with UUID: {}", &player.number, &player.uuid);
        player_number += 1;

        let new_data = Arc::clone(&data);
        {
            let mut game = new_data.lock().unwrap();
            game.add_player(player);
        }

        tokio::spawn(async move {
            let (reader, mut writer) = socket.split();

            let mut reader = BufReader::new(reader);
            let mut line = String::new();

            loop {
                tokio::select! {
                    result = reader.read_line(&mut line) => {
                        if result.unwrap() == 0 {
                            break;
                        }

                        match line.trim() {
                            "ping" => {
                                println!("pong!")
                            }

                            "hand" => {
                                // TODO: print out hand
                                println!("hand")
                            }

                            "start" => {
                                let mut game = new_data.lock().unwrap();
                                match game.generate_supply(Game::default_supply_list()) {
                                    Ok(()) => println!("Started game with default supply cards!"),
                                    Err(NotEnoughPlayers) => println!("Not enough players to start!"),
                                    _ => panic!("Unknown error while starting!")
                                }

                            }

                            _ => println!("Unknown command!")
                        }

                        tx.send((line.clone(), addr)).unwrap();
                        line.clear();
                    }

                    result = rx.recv() => {
                        let (msg, other_addr) = result.unwrap();

                        if addr != other_addr {
                            writer.write_all(msg.as_bytes()).await.unwrap();
                        }
                    }
                }
            }
        });
    }
}
