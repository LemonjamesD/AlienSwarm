pub mod game;

use std::io::{stdout};

use crossterm::{
    execute, Result,
    style::Print,
    terminal::{Clear, ClearType}
};

use game::Game;

#[tokio::main]
async fn main() -> Result<()> {
    execute!(
        stdout(),
        Clear(ClearType::All),
        Print("Hello"),
    )?;

    let game = Game::new();

    Ok(())
}
