use rumqttc::{AsyncClient, Event, MqttOptions, Packet, QoS};
use crate::state::AppState;
use crate::wiz::{color_json, off_json, resolve_color, send_wiz_command};

// ── MQTT Client & Subscriber Loop ─────────────────────────────────────────────

pub async fn run_mqtt(state: AppState) {
    let mut opts = MqttOptions::new("iot-bridge", "localhost", 1883);
    opts.set_keep_alive(std::time::Duration::from_secs(30));

    let (client, mut eventloop) = AsyncClient::new(opts, 64);
    client.subscribe("led/color", QoS::AtMostOnce).await.unwrap();
    println!(" MQTT listening on 'led/color'...");

    loop {
        match eventloop.poll().await {
            Ok(Event::Incoming(Packet::Publish(msg))) => {
                let color = String::from_utf8_lossy(&msg.payload).to_string();
                if let Some((r, g, b, is_on)) = resolve_color(&color) {
                    {
                        let mut led = state.led.write().unwrap();
                        led.color_name = color.clone();
                        led.r = r;
                        led.g = g;
                        led.b = b;
                        led.is_on = is_on;
                    }
                    let payload = if is_on { color_json(r, g, b) } else { off_json() };
                    let _ = send_wiz_command(&state.bulb_ip, &payload);
                    println!("📨 MQTT → LED: {}", color);
                } else {
                    println!(" Unknown MQTT color: '{}'", color);
                }
            }
            Ok(_) => {}
            Err(e) => {
                println!(" MQTT error: {}", e);
                break;
            }
        }
    }
}
