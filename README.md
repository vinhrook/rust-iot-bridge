#  IoT Protocol Bridge & Smart Lighting Hub (Rust)

> A high-performance, concurrent multi-protocol IoT Gateway written in **Rust** that bridges **Cloud IoT (MQTT)** and **Local Smart Hardware (WiZ UDP)** with an embedded real-time **Axum Web Dashboard**.

---

##  What is This Project?

In modern IoT environments, different systems speak different protocols:

- **The Cloud / IT World speaks MQTT:** Web dashboards, mobile apps, and cloud servers exchange lightweight, asynchronous JSON messages.
- **The Physical Hardware speaks UDP:** Smart devices (like WiZ LED lights) communicate over raw, local UDP network datagrams for zero-latency control.

###  The Solution: `rust-iot-bridge`
This project acts as the **universal translator and orchestrator**. 

When a color command is triggered from **anywhere** (a web dashboard, a mobile phone, or an MQTT message), this Rust bridge:
1. **Safely updates the central state** across multiple threads with zero race conditions.
2. **Sends low-latency UDP datagrams** directly to the physical hardware.

```
                  ┌───────────────────────────────────────────────────────────┐
                  │                 CONTROL INTERFACES                        │
                  │                                                           │
                  │   Modern Web UI                       MQTT Subscriber     │
                  │  (http://...:8080)                 (localhost:1883)       │
                  └─────────────┬───────────────────────────────┬─────────────┘
                                │                               │
                                ▼                               ▼
                  ┌───────────────────────────────────────────────────────────┐
                  │           RUST MULTI-THREADED CORE ENGINE                 │
                  │                                                           │
                  │         Arc<RwLock<LedState>> (Single Source of Truth)    │
                  │             • Tokio Asynchronous Event Loop               │
                  └─────────────────────────────┬─────────────────────────────┘
                                                │
                                                ▼ (Local UDP Socket :38899)
                                  ┌───────────────────────────┐
                                  │        Smart LED        │
                                  │   (Instant Hardware Flow) │
                                  └───────────────────────────┘
```

---

##  Project Architecture & Code Structure

```
rust-iot-bridge/src/
├── state.rs          
├── wiz.rs            
├── mqtt.rs           
├── web.rs            
└── main.rs           
```

### Module Breakdown:

| Module | Role | Description |
| :--- | :--- | :--- |
| **`state.rs`** | `LedState` & `AppState` | Uses `Arc<RwLock<>>` so multiple concurrent threads can safely read and write hardware state simultaneously without data corruption. |
| **`wiz.rs`** | UDP Networking | Encapsulates WiZ JSON protocol and RGB color conversions away from the web and MQTT layers. |
| **`mqtt.rs`** | Cloud / Message Broker | Asynchronously polls the Mosquitto MQTT broker using `rumqttc` on the Tokio runtime. |
| **`web.rs`** | User Interface & REST API | Serves a responsive, glassmorphism web dashboard and provides `/api/color/{color}` and `/api/status` endpoints. |
| **`main.rs`** | Service Orchestration | A clean coordinator that initializes logging and launches all concurrent services. |

---

## 🛠️ Technology Stack

- **Core Language:** [Rust](https://www.rust-lang.org/) (2024 Edition) — Memory safety with zero-cost abstractions.
- **Async Runtime:** [Tokio](https://tokio.rs/) — High-performance asynchronous event handling.
- **Web Framework:** [Axum](https://github.com/tokio-rs/axum) — Modular, ergonomic web server built on top of Hyper and Tower.
- **MQTT Engine:** [rumqttc](https://crates.io/crates/rumqttc) — Robust, pure-Rust MQTT client.
- **Hardware Networking:** `std::net::UdpSocket` — Direct datagram transmission with zero cloud overhead.
- **Logging & Diagnostics:** `tracing` & `tracing-subscriber` — Structured application telemetry.

---

## 🚀 Getting Started

### 1. Prerequisites

- **Rust toolchain:** `rustup` & `cargo` (Rust 1.80+)
- **MQTT Broker:** `mosquitto` running locally or on the local network:
  ```bash
  mosquitto -d
  ```
- **Smart Bulb:** Connected to your local WiFi subnet.

---

### 2. Configuration

Set your WiZ bulb's local IP in `src/main.rs`:
```rust
let bulb_ip = "10.226.20.xxxx".to_string(); // Replace with your bulb's local IP
```

```bash
# Build and start the gateway
cargo run
```

When started, you will see:
```text
====================================================
  IoT Bridge & Smart Lighting Hub v0.1.0
====================================================
 Dashboard  : http://localhost:8080
 MQTT Broker: localhost:1883 [topic: led/color]
 Target LED  : 10.226.209.74 (WiZ UDP 38899)
====================================================
```

---

##  How to Use & Test

### 1. Web Dashboard (Browser)
Open your browser to:
 **`http://localhost:8080`**

Click any color button (🔴 Red, 🟢 Green, 🔵 Blue, 🩷 Pink, 🟡 Yellow, 🩵 Cyan, ⚪ White, 🟠 Orange, 🟣 Purple, or ⚫ Off) to update the light and dashboard state immediately.

---

### 2. Terminal MQTT Control
Publish commands using any MQTT client (`mosquitto_pub`):

```bash
mosquitto_pub -h localhost -t "led/color" -m "pink"
mosquitto_pub -h localhost -t "led/color" -m "cyan"
mosquitto_pub -h localhost -t "led/color" -m "yellow"
mosquitto_pub -h localhost -t "led/color" -m "off"
```

---

##  REST API Reference

| Method | Endpoint | Description | Sample Response |
| :--- | :--- | :--- | :--- |
| `GET` | `/` | Serves the interactive web dashboard | HTML |
| `POST` | `/api/color/{color}` | Sets LED color (`red`, `green`, `blue`, `pink`, `off`, etc.) | `{"ok": true, "color": "pink"}` |
| `GET` | `/api/status` | Returns live hardware state & RGB values | `{"color": "pink", "r": 255, "g": 105, "b": 180, "is_on": true}` |

---

##  Security & Offline Resilience

- **100% Local LAN Communication:** Commands travel directly over local UDP sockets. Your lighting controls continue working even during complete internet outages.
- **Thread Isolation:** The web server and MQTT subscriber execute concurrently without blocking each other.
- **Memory Safety:** Written entirely in safe Rust, preventing memory leaks, buffer overflows, and null-pointer exceptions.

---

### Why Rust Better and where 
Metric	                   C / C++	          Python	             Rust 🦀
Raw Speed	               (Fastest)	      (Slow)	                    (Fastest)
Memory Usage	           (Minimal)   	    (Heavy)	                     (Minimal)
Memory Safety	           (Dangerous)	     (Safe)	                     (Safe)
Garbage Collector	       (None)	         (Yes, causes pauses)	       (None - zero latency)
Package Manager	           (Complex)	      (pip)	               (cargo - Best in class)

For IoT	                 (Hard to maintain safel)      (Too slow )      The Perfect Balance




##  License

Distributed under the **MIT License** / **Apache-2.0 License**.


