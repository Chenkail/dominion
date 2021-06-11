use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
    sync::broadcast
};

use dominion::game::prelude::*;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("localhost:8080").await.unwrap();
    let (tx, _rx) = broadcast::channel(10);

    let mut player_number = 0;

    loop {
        let (mut socket, addr) = listener.accept().await.unwrap();

        let tx = tx.clone();
        let mut rx = tx.subscribe();

        let player = Player::new_with_default_deck(player_number);
        player_number += 1;

        println!("Player #{} joined with UUID: {}", player.number, player.uuid);

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
                            "print" => {
                                println!("Hello!")
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
