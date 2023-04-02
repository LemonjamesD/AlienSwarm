use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode},
    Result,
};

pub struct Game {}

impl Game {
    pub fn new() -> Result<Game> {
        enable_raw_mode()?;
        Ok(Game {})
    }

    pub async fn game_loop(&mut self) -> Result<()> {
        tokio::spawn(async move { loop {
            // TODO: Add Logic and Stuff
        }})
    }
}

impl Drop for Game {
    fn drop(&mut self) {
        disable_raw_mode();
    }
}
