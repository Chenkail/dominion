use std::sync::{Arc, Mutex};

use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
    sync::broadcast
};

use dominion::game::prelude::*;
use dominion::error::DominionError::*;


fn single_recipient(player_number: usize) -> Recipients {
    vec![player_number]
}

fn everyone_but(player_count: usize, player_number: usize) -> Recipients {
    let mut v = (0..player_count).collect::<Vec<usize>>();
    v.remove(player_number);

    v
}

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("localhost:8080").await.unwrap();
    let (tx, _rx) = broadcast::channel(10);

    let data = Arc::new(Mutex::new(Game::new()));
    let mut player_count = 0;

    loop {
        let (mut socket, _addr) = listener.accept().await.unwrap();


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

        tokio::spawn(async move {
            let player_number = player_number;
            let (reader, mut writer) = socket.split();

            let mut reader = BufReader::new(reader);
            let mut line = String::new();

            loop {
                tokio::select! {
                    result = reader.read_line(&mut line) => {
                        if result.unwrap() == 0 {
                            break;
                        }

                        let mut command_parts = line.split_whitespace();
                        match command_parts.next().unwrap_or("Oops") {
                            "ping" => {
                                let recipients = single_recipient(player_number);
                                let message = "pong!\n".to_string();
                                tx.send((message, recipients)).unwrap();
                            }

                            "hand" => {
                                let game = new_data.lock().unwrap();
                                let player = &game.players[player_number];
                                let recipients = single_recipient(player_number);
                                let message = player.print_hand() + "\n";
                                tx.send((message, recipients)).unwrap();
                            }

                            "start" => {
                                let mut game = new_data.lock().unwrap();
                                if game.started {
                                    let recipients = single_recipient(player_number);
                                    let message = "Game has already started!\n".to_string();
                                    tx.send((message, recipients)).unwrap();
                                    continue;
                                }

                                let supply_list = Game::default_supply_list();

                                match game.generate_supply(supply_list) {
                                    Ok(()) => {
                                        game.started = true;
                                        let recipients = single_recipient(player_number);
                                        let message = "Started game with default supply cards!\n".to_string();
                                        tx.send((message, recipients)).unwrap();
                                    }
                                    Err(NotEnoughPlayers) => {
                                        let recipients = single_recipient(player_number);
                                        let message = "Not enough players to start!\n".to_string();
                                        tx.send((message, recipients)).unwrap();
                                    }
                                    _ => panic!("Unknown error while starting!")
                                }
                            }

                            _ => {
                                let recipients = single_recipient(player_number);
                                let message = "Unknown command!\n".to_string();
                                tx.send((message, recipients)).unwrap();
                            }
                        }

                        line.clear();
                    }

                    result = rx.recv() => {
                        let (msg, recipients) = result.unwrap();

                        if recipients.contains(&player_number) {
                            writer.write_all(msg.as_bytes()).await.unwrap();
                        }
                    }
                }
            }
        });
    }
}
