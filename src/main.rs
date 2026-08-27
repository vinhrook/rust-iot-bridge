mod mqtt;
mod state;
mod web;
mod wiz;

use std::sync::{Arc, RwLock};
use state::{AppState, LedState};

#[tokio::main]
async fn main() {
    // 1. Initialize logging
    tracing_subscriber::fmt::init();

    println!("====================================================");
    println!("  IoT Bridge & Smart Lighting Hub v0.1.0");
    println!("====================================================");
    println!(" Dashboard  : http://localhost:8080");
    println!(" MQTT Broker: localhost:1883 [topic: led/color]");
    println!(" Target LED  : 10.226.209.74 (WiZ UDP 38899)");
    println!("====================================================");

    //  Hardware Target IP 
    let bulb_ip = "10.226.209.74".to_string();  // Right Now change with fake ip

    // Shared thread-safe application state
    let app_state = AppState {
        led: Arc::new(RwLock::new(LedState::new())),
        bulb_ip,
    };

    // 1. Spawn Asynchronous MQTT Subscriber task on Tokio runtime
    let mqtt_state = app_state.clone();
    tokio::spawn(async move {
        mqtt::run_mqtt(mqtt_state).await;
    });

    // 2. Start Axum Web Server on main Tokio worker
    let router = web::create_router(app_state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, router).await.unwrap();
}
