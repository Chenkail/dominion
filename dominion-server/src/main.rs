use std::sync::{Arc, Mutex};

use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
    sync::broadcast
};

use dominion::game::prelude::*;
use dominion::error::DominionError::*;

type Recipients = Option<Vec<usize>>;

fn single_recipient(player_number: usize) -> Recipients {
    Some(vec![player_number])
}

fn everyone_but(player_count: usize, player_number: usize) -> Recipients {
    let mut v = (0..player_count).collect::<Vec<usize>>();
    v.remove(player_number);
    Some(v)
}

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("localhost:8080").await.unwrap();
    let (tx, _rx) = broadcast::channel(10);

    let data = Arc::new(Mutex::new(Game::new()));
    let mut player_count = 0;

    loop {
        let (mut socket, addr) = listener.accept().await.unwrap();

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
                                tx.send(("pong!\n", recipients)).unwrap();
                            }

                            "hand" => {
                                // TODO: send hand message back to player
                                let game = new_data.lock().unwrap();
                                let player = &game.players[player_number];
                                player.print_hand();
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

                        line.clear();
                    }

                    result = rx.recv() => {
                        let (msg, recipients) = result.unwrap();

                        if recipients.unwrap().contains(&player_number) {
                            writer.write_all(msg.as_bytes()).await.unwrap();
                        }
                    }
                }
            }
        });
    }
}
