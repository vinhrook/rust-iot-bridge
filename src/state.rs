use std::sync::{Arc, RwLock};

// ── Shared LED State ──────────────────────────────────────────────────────────
// Single source of truth. All tasks read/write through Arc<RwLock<>>.
#[derive(Debug, Clone)]
pub struct LedState {
    pub color_name: String,
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub is_on: bool,
}

impl LedState {
    pub fn new() -> Self {
        Self {
            color_name: "off".to_string(),
            r: 0,
            g: 0,
            b: 0,
            is_on: false,
        }
    }
}

// ── App State (shared across Axum routes & background workers) ────────────────
#[derive(Clone)]
pub struct AppState {
    pub led: Arc<RwLock<LedState>>,
    pub bulb_ip: String,
}
