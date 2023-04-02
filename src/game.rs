use crossterm::{terminal::{enable_raw_mode, disable_raw_mode}, Result};

pub struct Game {}

impl Game {
    pub fn new() -> Result<Game> {
        enable_raw_mode()?;
        Ok(Game {})
    }
}

impl Drop for Game {
    fn drop(&mut self) {
        disable_raw_mode();
    }
}
