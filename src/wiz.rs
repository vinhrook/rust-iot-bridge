use std::net::UdpSocket;

// ── WiZ Smart Lighting UDP Protocol Driver 
/// Sends raw JSON datagram to WiZ bulb on UDP port 38899
pub fn send_wiz_command(bulb_ip: &str, json_payload: &str) -> std::io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    socket.send_to(json_payload.as_bytes(), format!("{}:38899", bulb_ip))?;
    Ok(())
}

/// Builds JSON payload for RGB color
pub fn color_json(r: u8, g: u8, b: u8) -> String {
    format!(
        r#"{{"method":"setPilot","params":{{"r":{},"g":{},"b":{}}}}}"#,
        r, g, b
    )
}

/// Builds JSON payload for turning off the light
pub fn off_json() -> String {
    r#"{"method":"setPilot","params":{"state":false}}"#.to_string()
}

/// Resolves human-readable color string to RGB values and power state
pub fn resolve_color(color: &str) -> Option<(u8, u8, u8, bool)> {
    match color {
        "red"    => Some((255, 0,   0,   true)),
        "green"  => Some((0,   255, 0,   true)),
        "blue"   => Some((0,   0,   255, true)),
        "pink"   => Some((255, 105, 180, true)),
        "yellow" => Some((255, 255, 0,   true)),
        "cyan"   => Some((0,   255, 255, true)),
        "white"  => Some((255, 255, 255, true)),
        "orange" => Some((255, 100, 0,   true)),
        "purple" => Some((128, 0,   255, true)),
        "off"    => Some((0,   0,   0,   false)),
        _        => None,
    }
}
