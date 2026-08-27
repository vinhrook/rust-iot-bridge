use std::sync::{Arc, RwLock};


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


#[derive(Clone)]
pub struct AppState {
    pub led: Arc<RwLock<LedState>>,
    pub bulb_ip: String,
}
