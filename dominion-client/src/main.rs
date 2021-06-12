use dominion_server::api::{ClientMessage, ServerMessage};
use futures::prelude::*;
use tokio::net::TcpStream;
use tokio_serde::formats::*;
use tokio_util::codec::{FramedRead, FramedWrite, LengthDelimitedCodec};

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
                _ => {
                    println!("Uh oh!")
                }
            }
        }
    });

    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Error while reading user input!");
        let mut command_parts = input.split_whitespace();
        match command_parts.next().unwrap_or("Oops") {
            "ping" => {
                serialized
                .send(serde_json::to_value(&ClientMessage::Ping).unwrap())
                .await
                .unwrap();
            }
            _ => println!("Couldn't understand input!")
        }
    }
}
