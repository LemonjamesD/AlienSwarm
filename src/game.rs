use crate::systems::blocks::{dirt::DirtBlock, air::AirBlock};
use crate::systems::human::Human;
use crate::systems::Thing;

use std::io::stdout;
use std::time::Duration;
use std::io::Write;
use std::process::exit;
use std::sync::Arc;

use crossterm::{
    ExecutableCommand,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
    execute, queue,
    style::Print,
    terminal::{Clear, ClearType},
    Result,
    cursor::{Hide, Show, MoveTo, MoveToColumn},
    event::read,
    event::Event,
    event::KeyCode::Char
};

use tokio::time::{Instant, sleep};
use tokio::sync::Mutex;
use tracing::info;

macro_rules! str_idx {
    ($name:expr, $i:expr, $expr:expr) => {
        $name.replace_range($i..$i+1, $expr)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn str_idx_test() {
        let spaces = {
            let mut s = String::new();
            for _ in 0..4 {
                s.push(' ');
            }
            s
        };
        let mut str = spaces;
        str_idx!(str, 3, "Hello");
        assert_eq!("   Hello".to_string(), str);
    }
}

pub struct Game {}

impl Game {
    pub fn new() -> Result<Game> {
        execute!(stdout(), EnterAlternateScreen, Hide)?;
        enable_raw_mode()?;
        Ok(Game {})
    }

    pub async fn game_loop(&mut self) -> Result<()> {
        let tiles: Vec<Vec<Thing>> = vec![
            (|x| { 
                let mut vec = vec![];
                for _ in 0..5 {
                    for _ in 0..5 {
                        vec.push(Thing::Air(x));
                    }
                    vec.push(Thing::Newline);
                }
                return vec;
            })(AirBlock::new()),
            (|x| { 
                let mut vec = vec![];
                for _ in 0..5 {
                    for _ in 0..5 {
                        vec.push(Thing::Air(x));
                    }
                    vec.push(Thing::Newline);
                }
                vec.pop();
                vec.pop();
                vec.push(Thing::Human(Human::new()));
                return vec;
            })(AirBlock::new()),
            (|x| { 
                let mut vec = vec![];
                for _ in 0..5 {
                    for _ in 0..5 {
                        vec.push(Thing::Dirt(x));
                    }
                    vec.push(Thing::Newline);
                }
                return vec;
            })(DirtBlock::new())
        ];
        let spaces = {
            let mut s = String::new();
            for _ in 0..30 {
                s.push(' ');
            }
            s
        };
        let mut instant = Instant::now();
        let mut fps = 0;
        let mut fps_display = 0;
        let mut stdout = stdout();
        execute!(stdout, Clear(ClearType::FromCursorDown))?;
        let mut stdout = Arc::new(Mutex::new(stdout));
        let stdout_a = stdout.clone();
        tokio::spawn(async move { loop {
            match read().unwrap() {
                Event::Key(event) => match event.code {
                    Char(c) => match c {
                        'q' => {
                            let stdout_a_a = stdout_a.clone();
                            let mut mutex_stdout = (stdout_a_a.lock()).await;
                            mutex_stdout.execute(LeaveAlternateScreen).unwrap().execute(Show).unwrap();
                            disable_raw_mode().unwrap();
                            exit(0);
                        },
                        _ => ()
                    }
                    _ => ()
                },
                _ => ()
            }
        }});
        let stdout_b = stdout.clone();
        tokio::spawn(async move { loop {
            let mut frame = spaces.clone();
            for (i, tile) in (&tiles[2]).iter().enumerate() {
                let _ = match tile {
                    Thing::Human(_) => str_idx!(frame, i, "☺"),
                    Thing::Dirt(_) => str_idx!(frame, i, "D"),
                    Thing::Air(_) => (),
                    Thing::Newline => str_idx!(frame, i, "\n"),
                    _ => ()
                };
            }
            for (i, tile) in (&tiles[1]).iter().enumerate() {
                let _ = match tile {
                    Thing::Human(_) => str_idx!(frame, i, "☺"),
                    Thing::Dirt(_) => str_idx!(frame, i, "D"),
                    Thing::Air(_) => (),
                    Thing::Newline => str_idx!(frame, i, "\n"),
                    _ => ()
                };
            }
            frame = frame.replace("\n", "\r\n");
            if instant.elapsed().as_secs() >= 1 {
                fps_display = fps;
                fps = 0;
                instant = Instant::now();
            } else {
                fps += 1;
            }
            queue!(*stdout_b.lock().await, MoveTo(0,0), Print(format!("{fps_display} fps\r\n")), Print(frame.clone())).unwrap();
            (*stdout_b.lock().await).flush().unwrap();
            sleep(Duration::from_nanos(16666666)).await;
        }}).await;

        Ok(())
    }
}

impl Drop for Game {
    fn drop(&mut self) {
        execute!(stdout(), LeaveAlternateScreen, Show).unwrap();
        disable_raw_mode().unwrap();
    }
}
