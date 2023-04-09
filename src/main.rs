#![feature(trait_alias)]

pub mod systems;
pub mod game;

use game::Game;

use crossterm::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let file_appender = tracing_appender::rolling::hourly("./", "game.logs");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    tracing_subscriber::fmt()
        .with_writer(non_blocking)
        .init();
    
    let mut game = Game::new()?;

    game.game_loop().await?;

    Ok(())
}
