use crate::systems::blocks::{dirt::DirtBlock, air::AirBlock};
use crate::systems::human::Human;
use crate::systems::Thing;

use std::io::{stdout, Stdout};
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
    event::KeyCode
};

use tokio::time::{Instant, sleep};
use tokio::sync::{Mutex, RwLock};
use tracing::{info, debug};

trait NthChar {
    fn replace_nth(&mut self, idx: usize, newchar: char);
}

impl NthChar for String {
    fn replace_nth(&mut self, idx: usize, newchar: char) {
        *self = self.chars().enumerate().map(|(i,c)| if i == idx { newchar } else { c }).collect::<String>();
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

#[derive(Clone)]
pub struct Game {
    current_tile: i32,
}

impl Game {
    pub fn new() -> Result<Game> {
        execute!(stdout(), EnterAlternateScreen, Hide)?;
        execute!(stdout(), Clear(ClearType::FromCursorDown))?;
        enable_raw_mode()?;
        Ok(Game {
            current_tile: 1
        })
    }

    pub async fn game_loop(&mut self) -> Result<()> {
        let tiles = vec![
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
                vec.push(Thing::Newline);
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

        for x in &tiles {
            debug!("{}", x.len());
        }
 
        let stdout_ = Arc::new(Mutex::new(stdout()));
        let new_self = Arc::new(Mutex::new(self.clone()));
        let tiles = Arc::new(RwLock::new(tiles));

        Game::get_input(new_self.clone(), stdout_.clone()).await;
        Game::draw_frame(&*self, stdout_.clone(), tiles).await;

        info!("Called main functions");
        

        Ok(())
    }

    pub async fn get_input(self_: Arc<Mutex<Self>>, stdout: Arc<Mutex<Stdout>>) {
        info!("get_input called");
        tokio::spawn(async move { loop {
            match read().unwrap() {
                Event::Key(event) => match event.code {
                    KeyCode::Char(c) => match c {
                        'q' => {
                            let self_a = self_.clone();
                            let mut mut_self = self_a.lock().await;
                            mut_self.current_tile = std::cmp::max(mut_self.current_tile - 1, 1);
                        },
                        'e' => {
                            let self_a = self_.clone();
                            let mut mut_self = self_a.lock().await;
                            mut_self.current_tile += 1;
                        }
                        _ => ()
                    },
                    KeyCode::Esc => {
                        let mut mutex_stdout = stdout.lock().await;
                        mutex_stdout.execute(LeaveAlternateScreen).unwrap().execute(Show).unwrap();
                        disable_raw_mode().unwrap();
                        exit(0);
                    }
                    _ => ()
                },
                _ => ()
            };
            info!("get_input loop running");
        }});
    }

    pub async fn draw_frame(self_: &Self, stdout: Arc<Mutex<Stdout>>, tiles: Arc<RwLock<Vec<Vec<Thing>>>>) {
        info!("draw_frame called");

        let spaces = {
            let mut s = String::new();
            for _ in 0..30 {
                s.push(' ');
            }
            s
        };
        debug!("Spaces: {}", spaces.len());

        let mut instant = Instant::now();
        let mut fps = 0;
        let mut fps_display = 0;
        let self_ = self_.clone();
        let tiles_len = (tiles.clone().read().await).len().clone();
        debug!("Gets through main block");
        tokio::spawn(async move { loop {
            let mut frame = spaces.clone();
            let curr_tile = std::cmp::min(tiles_len-1, (self_.current_tile + 1) as usize) as usize;
            let curr_tiles = tiles.clone(); 
            let curr_tiles = (*curr_tiles.read().await).clone();
            debug!("{}", frame.len());
            debug!("{}", tiles_len);
            for (i, tile) in (curr_tiles[curr_tile]).iter().enumerate() {
                let _ = match tile {
                    Thing::Human(e) => frame.replace_nth(i, format!("{e}").as_str().chars().nth(0).unwrap()),
                    Thing::Dirt(e) => frame.replace_nth(i, format!("{e}").as_str().chars().nth(0).unwrap()),
                    Thing::Air(e) => (),
                    Thing::Newline => frame.replace_nth(i, "\n".chars().nth(0).unwrap()),
                    _ => ()
                };
            }
            debug!("Got through first draw_loop: {}", frame.len());
            for (i, tile) in (curr_tiles[curr_tile - 1]).iter().enumerate() {
                let _ = match tile {
                    Thing::Human(e) => frame.replace_nth(i, format!("{e}").as_str().chars().nth(0).unwrap()),
                    Thing::Dirt(e) => frame.replace_nth(i, format!("{e}").as_str().chars().nth(0).unwrap()),
                    Thing::Air(e) => (),
                    Thing::Newline => frame.replace_nth(i, "\n".chars().nth(0).unwrap()),
                    _ => ()
                };
            }
            debug!("Got through second draw loop");
            frame = frame.replace("\n", "\r\n");
            if instant.elapsed().as_secs() >= 1 {
                fps_display = fps;
                fps = 0;
                instant = Instant::now();
            } else {
                fps += 1;
            }
            queue!(*stdout.lock().await, MoveTo(0,0), Print(format!("{fps_display} fps\r\n")), Print(frame.clone())).unwrap();
            (*stdout.lock().await).flush().unwrap();
            sleep(Duration::from_nanos(16666666)).await;
            info!("draw_frame loop running");
        }}).await;
    }
}

impl Drop for Game {
    fn drop(&mut self) {
        execute!(stdout(), LeaveAlternateScreen, Show).unwrap();
        disable_raw_mode().unwrap();
    }
}
