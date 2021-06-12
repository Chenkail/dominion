use dominion_server::api::{ClientMessage, ServerMessage};
use futures::prelude::*;
use tokio::net::TcpStream;
use tokio_serde::formats::*;
use tokio_util::codec::{FramedRead, FramedWrite, LengthDelimitedCodec};

enum InputMode {
    Console,
    Chat,
}

#[tokio::main]
pub async fn main() {
    // Bind a server socket
    let socket = TcpStream::connect("localhost:8080").await.unwrap();

    let socket = socket.into_std().unwrap();
    let socket2 = socket.try_clone().unwrap();
    let socket = TcpStream::from_std(socket).unwrap();
    let socket2 = TcpStream::from_std(socket2).unwrap();

    let length_delimited = FramedRead::new(socket, LengthDelimitedCodec::new());
    let mut deserialized = tokio_serde::SymmetricallyFramed::new(
        length_delimited,
        SymmetricalJson::<ServerMessage>::default(),
    );

    let length_delimited = FramedWrite::new(socket2, LengthDelimitedCodec::new());
    let mut serialized =
        tokio_serde::SymmetricallyFramed::new(length_delimited, SymmetricalJson::default());

    // Spawn a task to handle incoming messages
    tokio::spawn(async move {
        while let Some(msg) = deserialized.try_next().await.unwrap() {
            match msg {
                ServerMessage::PingResponse => {
                    println!("Pong!");
                }
                ServerMessage::ChatMessage{ author, message } => {
                    println!("Player {}: \"{}\"", author, message)
                }
                _ => {
                    println!("Got a message from the server that the client couldn't understand!")
                }
            }
        }
    });

    let mut input_mode = InputMode::Console;

    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Error while reading user input!");
        let trimmed = input.trim_matches(char::is_whitespace);
        match input_mode {
            InputMode::Console => {
                let mut command_parts = trimmed.split_whitespace();
                match command_parts.next().unwrap_or("Oops") {
                    "ping" => {
                        serialized
                        .send(serde_json::to_value(&ClientMessage::Ping).unwrap())
                        .await
                        .unwrap();
                    }
                    "chat" => {
                        input_mode = InputMode::Chat;
                    }
                    "quit" => {
                        panic!("Exiting client")
                    }
                    "start" => {
                        serialized
                        .send(serde_json::to_value(&ClientMessage::StartGame).unwrap())
                        .await
                        .unwrap();
                    }
                    _ => println!("Couldn't understand input!")
                }
            }
            InputMode::Chat => {
                match trimmed {
                    "" => {}
                    "exit" => {
                        println!("Leaving chat mode!");
                        input_mode = InputMode::Console;
                    }
                    _ => {
                        serialized
                        .send(serde_json::to_value(&ClientMessage::ChatMessage{ message: trimmed.to_string() }).unwrap())
                        .await
                        .unwrap();
                    }
                }
            }
        }

    }
}
