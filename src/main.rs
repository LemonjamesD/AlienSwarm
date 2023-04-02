use std::io::{stdout};

use crossterm::{
    execute, Result,
    style::Print,
};

#[tokio::main]
async fn main() -> Result<()> {
    execute!(
        stdout(),
        Print("Hello")
    )?;

    Ok(())
}
