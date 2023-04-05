#![feature(trait_alias)]

pub mod systems;
pub mod game;

use game::Game;

use crossterm::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let mut game = Game::new()?;

    game.game_loop().await?;

    Ok(())
}
