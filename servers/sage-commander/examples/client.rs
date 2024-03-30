use futures::{SinkExt, StreamExt};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

use shared_sage_commander as cmds;

#[tokio::main]
async fn main() {
    let url = url::Url::parse("ws://127.0.0.1:3030/cmd").unwrap();
    let (ws_stream, _response) = connect_async(url).await.unwrap();

    // dbg!(_response);
    let (mut tx, mut rx) = ws_stream.split();

    let cmd = cmds::Command::Inquiry(cmds::Inquiry::Fleet(
        "771Sgp2yb1h3XsCrQjFLRq5L74ZX6qD8wzbZmjGeMxtF".to_string(),
    ));
    let encoded: Vec<u8> = borsh::to_vec(&cmd).unwrap();

    let message = Message::binary(encoded);
    tx.send(message).await.unwrap();

    while let Some(message) = rx.next().await {
        let message = message.unwrap();
        let bytes = message.into_data();
        let reply = borsh::from_slice::<cmds::Reply>(&bytes).unwrap();

        dbg!(reply);
    }
}
