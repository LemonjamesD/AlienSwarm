#![feature(trait_alias)]

pub mod systems;
pub mod game;

use std::io::stdout;

use crossterm::{
    execute,
    style::Print,
    terminal::{Clear, ClearType},
    Result,
};

use game::Game;

#[tokio::main]
async fn main() -> Result<()> {
    let mut game = Game::new()?;

    game.game_loop().await?;

    Ok(())
}
