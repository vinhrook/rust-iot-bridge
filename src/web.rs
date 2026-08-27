use axum::{
    extract::{Path, State},
    response::Html,
    routing::{get, post},
    Json, Router,
};
use crate::state::AppState;
use crate::wiz::{color_json, off_json, resolve_color, send_wiz_command};

pub fn create_router(app_state: AppState) -> Router {
    Router::new()
        .route("/", get(dashboard))
        .route("/api/color/{color}", post(set_color))
        .route("/api/status", get(get_status))
        .with_state(app_state)
}

async fn dashboard() -> Html<&'static str> {
    Html(DASHBOARD_HTML)
}

async fn set_color(
    Path(color): Path<String>,
    State(state): State<AppState>,
) -> Json<serde_json::Value> {
    if let Some((r, g, b, is_on)) = resolve_color(&color) {
        let payload = if is_on { color_json(r, g, b) } else { off_json() };
        {
            let mut led = state.led.write().unwrap();
            led.color_name = color.clone();
            led.r = r;
            led.g = g;
            led.b = b;
            led.is_on = is_on;
        }
        println!(" Dashboard → LED: {}", color);
        match send_wiz_command(&state.bulb_ip, &payload) {
            Ok(()) => Json(serde_json::json!({ "ok": true, "color": color })),
            Err(e) => Json(serde_json::json!({ "ok": false, "error": e.to_string() })),
        }
    } else {
        Json(serde_json::json!({ "ok": false, "error": "unknown color" }))
    }
}

async fn get_status(State(state): State<AppState>) -> Json<serde_json::Value> {
    let led = state.led.read().unwrap();
    Json(serde_json::json!({
        "color": led.color_name,
        "r": led.r,
        "g": led.g,
        "b": led.b,
        "is_on": led.is_on,
    }))
}

// ── Web Dashboard Embedded 
const DASHBOARD_HTML: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<title>💡 LED Controller</title>
<style>
  @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;600;700&display=swap');
  * { box-sizing: border-box; margin: 0; padding: 0; }
  body {
    font-family: 'Inter', sans-serif;
    background: radial-gradient(ellipse at top, #1a1a2e 0%, #0a0a0f 70%);
    color: #e0e0e0;
    min-height: 100vh;
    display: flex;
    align-items: center;
    justify-content: center;
  }
  .card {
    background: rgba(255,255,255,0.04);
    backdrop-filter: blur(20px);
    border: 1px solid rgba(255,255,255,0.08);
    border-radius: 28px;
    padding: 44px 36px;
    text-align: center;
    max-width: 480px;
    width: 92%;
    box-shadow: 0 30px 80px rgba(0,0,0,0.6);
  }
  h1 { font-size: 1.7rem; font-weight: 700; margin-bottom: 6px; }
  .subtitle { color: rgba(255,255,255,0.35); font-size: 0.8rem; margin-bottom: 28px; letter-spacing: 0.5px; }
  .status-bar {
    background: rgba(255,255,255,0.06);
    border-radius: 14px;
    padding: 14px 20px;
    margin-bottom: 28px;
    font-size: 0.88rem;
    display: flex;
    align-items: center;
    justify-content: center;
rust-iot-bridge    gap: 12px;
    border: 1px solid rgba(255,255,255,0.07);
  }
  .dot {
    width: 14px; height: 14px;
    border-radius: 50%;
    background: #333;
    transition: background 0.5s, box-shadow 0.5s;
  }
  .grid {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 11px;
    margin-bottom: 12px;
  }
  .btn-off { grid-column: span 3; }
  button {
    border: none;
    border-radius: 16px;
    padding: 20px 8px;
    font-size: 0.82rem;
    font-weight: 600;
    cursor: pointer;
    letter-spacing: 0.4px;
    transition: transform 0.15s ease, box-shadow 0.15s ease;
    color: white;
    text-shadow: 0 1px 4px rgba(0,0,0,0.5);
  }
  button:hover { transform: translateY(-3px); box-shadow: 0 10px 28px rgba(0,0,0,0.5); }
  button:active { transform: scale(0.95); }
  button.active { outline: 2.5px solid rgba(255,255,255,0.75); outline-offset: 3px; }
  .btn-red    { background: linear-gradient(135deg, #ff416c, #c0392b); }
  .btn-green  { background: linear-gradient(135deg, #11998e, #38ef7d); color: #003; text-shadow:none; }
  .btn-blue   { background: linear-gradient(135deg, #2980b9, #6dd5fa); }
  .btn-pink   { background: linear-gradient(135deg, #f953c6, #b91d73); }
  .btn-yellow { background: linear-gradient(135deg, #f7971e, #ffd200); color: #222; text-shadow:none; }
  .btn-cyan   { background: linear-gradient(135deg, #00c6ff, #0072ff); }
  .btn-white  { background: linear-gradient(135deg, #bdc3c7, #ffffff); color: #222; text-shadow:none; }
  .btn-orange { background: linear-gradient(135deg, #f46b45, #eea849); }
  .btn-purple { background: linear-gradient(135deg, #7303c0, #a040f0); }
  .btn-off    { background: linear-gradient(135deg, #1a1a1a, #3d3d3d); padding: 16px; }
  .footer { margin-top: 22px; font-size: 0.72rem; color: rgba(255,255,255,0.2); letter-spacing: 0.5px; }
</style>
</head>
<body>
<div class="card">
  <h1>💡 LED Control</h1>
  <p class="subtitle">RUST · AXUM · MQTT · WIZ</p>

  <div class="status-bar">
    <div class="dot" id="dot"></div>
    <span id="status-text">Connecting...</span>
  </div>

  <div class="grid">
    <button class="btn-red"    onclick="setColor('red')">🔴 Red</button>
    <button class="btn-green"  onclick="setColor('green')">🟢 Green</button>
    <button class="btn-blue"   onclick="setColor('blue')">🔵 Blue</button>
    <button class="btn-pink"   onclick="setColor('pink')">🩷 Pink</button>
    <button class="btn-yellow" onclick="setColor('yellow')">🟡 Yellow</button>
    <button class="btn-cyan"   onclick="setColor('cyan')">🩵 Cyan</button>
    <button class="btn-white"  onclick="setColor('white')">⚪ White</button>
    <button class="btn-orange" onclick="setColor('orange')">🟠 Orange</button>
    <button class="btn-purple" onclick="setColor('purple')">🟣 Purple</button>
    <button class="btn-off"    onclick="setColor('off')">⚫ Turn Off</button>
  </div>

  <p class="footer">Built with Rust • Open Source</p>
</div>
<script>
const colorHex = {
  red:'#ff416c', green:'#38ef7d', blue:'#6dd5fa', pink:'#f953c6',
  yellow:'#ffd200', cyan:'#00c6ff', white:'#ffffff',
  orange:'#f46b45', purple:'#a040f0', off:'#333'
};

async function setColor(color) {
  document.querySelectorAll('button').forEach(b => b.classList.remove('active'));
  event.target.classList.add('active');
  await fetch(`/api/color/${color}`, { method: 'POST' });
  await loadStatus();
}

async function loadStatus() {
  try {
    const res = await fetch('/api/status');
    const d = await res.json();
    const dot = document.getElementById('dot');
    dot.style.background = colorHex[d.color] || '#333';
    dot.style.boxShadow = d.is_on ? `0 0 14px ${colorHex[d.color]}` : 'none';
    document.getElementById('status-text').textContent =
      d.is_on ? `Color: ${d.color.toUpperCase()}` : 'Turned Off';
  } catch { document.getElementById('status-text').textContent = 'Connection error'; }
}

loadStatus();
setInterval(loadStatus, 2000);
</script>
</body>
</html>"#;
