use futures_util::FutureExt;
use rust_socketio::{
    asynchronous::{Client, ClientBuilder},
    Payload, Socket,
};
use serde_json::json;
use std::{sync::Arc, thread, time::Duration};
use tokio::{signal, sync::Mutex};

#[tokio::main]
async fn main() {
    let auth = json!({"token": "p8q27jwx"});
    // https://zarena-dev1.zinza.com.vn
    // http://localhost:3000
    // Connect to the WebSocket
    let socket = Arc::new(Mutex::new(
        ClientBuilder::new("https://zarena-dev1.zinza.com.vn")
        .auth(auth)
            .on("error", |err, _| {
                async move { eprintln!("Error: {:#?}", err) }.boxed()
            })
            .on("message", |payload: Payload, socket: Client| {
                let socket = socket.clone();
                async move {
                    match payload {
                        Payload::Text(values) => println!("Received: {:#?}", values),
                        Payload::Binary(bin_data) => println!("Received bytes: {:#?}", bin_data),
                        Payload::String(str) => println!("Received: {}", str),
                    }
                }
                .boxed()
            })
            .connect()
            .await
            .expect("Connection failed"),
    ));
    thread::sleep(Duration::from_secs(1));

  
    // Emit the "join" event
    {
        let socket_clone = Arc::clone(&socket);
        tokio::spawn(async move {
            let socket = socket_clone.lock().await;
            socket
                .emit("join", json!({}))
                .await
                .expect("Server unreachable");
        });
    }

    println!("Connected to the server. Press Ctrl+C to disconnect.");

    // Wait for Ctrl+C signal
    signal::ctrl_c().await.expect("Failed to listen for Ctrl+C");

    // Perform disconnection
    {
        let socket_clone = Arc::clone(&socket);
        let _ = tokio::spawn(async move {
            let socket = socket_clone.lock().await;
            // Check if the library has a disconnect method
            if let Err(e) = socket.disconnect().await {
                eprintln!("Disconnect failed: {:#?}", e);
            }
        })
        .await; // Wait for the disconnection task to complete
    }

    println!("Disconnected from the server.");
}
