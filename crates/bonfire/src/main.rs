use std::env;

use async_std::net::TcpListener;
use revolt_presence::clear_region;

#[macro_use]
extern crate log;

pub mod config;
pub mod events;

mod database;
mod websocket;

#[async_std::main]
async fn main() {
    // Configure requirements for Bonfire. Check
    revolt_config::configure!(events);
    database::connect().await;

    // Clean up the current region information.
    let no_clear_region = env::var("NO_CLEAR_PRESENCE").unwrap_or_else(|_| "0".into()) == "1";
    if !no_clear_region {
        clear_region(None).await;
    }

    // Setup a TCP listener to accept WebSocket connections on.
    // By default, we bind to port 14703 on all interfaces.
    let port = env::var("PORT").unwrap_or_else(|_| "14705".into());
	let bind = format!("0.0.0.0:{}", port);
	info!("Listening on {bind}");
    let try_socket = TcpListener::bind(bind).await;
	info!("Bonfire WebSocket server running at 0.0.0.0:14705");
    let listener = try_socket.expect("Failed to bind");

    // Start accepting new connections and spawn a client for each connection.
    while let Ok((stream, addr)) = listener.accept().await {
        async_std::task::spawn(async move {
            info!("User connected from {addr:?}");
            websocket::client(database::get_db(), stream, addr).await;
            info!("User disconnected from {addr:?}");
        });
    }
}
