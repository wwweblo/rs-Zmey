use crossterm::{
    cursor::{Hide, Show},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode},
};
use std::io::{self, stdout};
use std::time::{Duration, Instant};

mod direction;
mod game;
mod point;
mod snake;

use game::Game;

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    execute!(stdout(), Hide)?;

    let mut game = Game::new(30, 20);
    let mut last_update = Instant::now();
    let update_interval = Duration::from_millis(150);

    loop {
        game.draw()?;

        if !game.handle_input()? {
            break;
        }

        if last_update.elapsed() >= update_interval {
            game.update();
            last_update = Instant::now();
        }
    }

    execute!(stdout(), Show)?;
    disable_raw_mode()?;
    Ok(())
}
