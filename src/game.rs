use crossterm::{
    cursor::MoveTo,
    event::{poll, read, Event, KeyCode, KeyEvent},
    execute,
    style::{Color, Print, SetForegroundColor},
    terminal::{Clear, ClearType},
};
use rand::Rng;
use std::collections::VecDeque;
use std::io::{self, stdout, Write};
use std::time::Duration;

use crate::direction::Direction;
use crate::point::Point;
use crate::snake::Snake;

pub struct Game {
    pub snake: Snake,
    pub food: Point,
    pub width: i32,
    pub height: i32,
    pub score: u32,
    pub game_over: bool,
}

impl Game {
    pub fn new(width: i32, height: i32) -> Self {
        let snake = Snake::new(width / 2, height / 2);
        let food = Self::generate_food(width, height, &snake.body);
        Game {
            snake,
            food,
            width,
            height,
            score: 0,
            game_over: false,
        }
    }

    pub fn generate_food(width: i32, height: i32, snake_body: &VecDeque<Point>) -> Point {
        let mut rng = rand::thread_rng();
        loop {
            let food = Point {
                x: rng.gen_range(0..width),
                y: rng.gen_range(0..height),
            };
            if !snake_body.contains(&food) {
                return food;
            }
        }
    }

    pub fn update(&mut self) {
        if self.game_over {
            return;
        }

        self.snake.move_forward();

        if self.snake.check_collision(self.width, self.height) {
            self.game_over = true;
            return;
        }

        if self.snake.eat_food(self.food) {
            self.score += 10;
            self.food = Self::generate_food(self.width, self.height, &self.snake.body);
        }
    }

    pub fn draw(&self) -> io::Result<()> {
        execute!(stdout(), Clear(ClearType::All))?;

        // Draw boundaries
        for x in 0..self.width + 2 {
            execute!(
                stdout(),
                MoveTo(x as u16, 0),
                SetForegroundColor(Color::White),
                Print("─")
            )?;
            execute!(
                stdout(),
                MoveTo(x as u16, (self.height + 1) as u16),
                SetForegroundColor(Color::White),
                Print("─")
            )?;
        }
        for y in 0..self.height + 2 {
            execute!(
                stdout(),
                MoveTo(0, y as u16),
                SetForegroundColor(Color::White),
                Print("│")
            )?;
            execute!(
                stdout(),
                MoveTo((self.width + 1) as u16, y as u16),
                SetForegroundColor(Color::White),
                Print("│")
            )?;
        }

        // Draw corners
        execute!(
            stdout(),
            MoveTo(0, 0),
            SetForegroundColor(Color::White),
            Print("┌")
        )?;
        execute!(
            stdout(),
            MoveTo((self.width + 1) as u16, 0),
            SetForegroundColor(Color::White),
            Print("┐")
        )?;
        execute!(
            stdout(),
            MoveTo(0, (self.height + 1) as u16),
            SetForegroundColor(Color::White),
            Print("└")
        )?;
        execute!(
            stdout(),
            MoveTo((self.width + 1) as u16, (self.height + 1) as u16),
            SetForegroundColor(Color::White),
            Print("┘")
        )?;

        // Draw snake
        for (i, segment) in self.snake.body.iter().enumerate() {
            let symbol = if i == 0 { "●" } else { "○" };
            let color = if i == 0 {
                Color::Green
            } else {
                Color::DarkGreen
            };
            execute!(
                stdout(),
                MoveTo((segment.x + 1) as u16, (segment.y + 1) as u16),
                SetForegroundColor(color),
                Print(symbol)
            )?;
        }

        // Draw food
        execute!(
            stdout(),
            MoveTo((self.food.x + 1) as u16, (self.food.y + 1) as u16),
            SetForegroundColor(Color::Red),
            Print("◆")
        )?;

        // Draw score
        execute!(
            stdout(),
            MoveTo(0, (self.height + 2) as u16),
            SetForegroundColor(Color::White),
            Print(format!(
                "Score: {} | Length: {}",
                self.score,
                self.snake.body.len()
            ))
        )?;

        if self.game_over {
            execute!(
                stdout(),
                MoveTo(0, (self.height + 3) as u16),
                SetForegroundColor(Color::Red),
                Print("GAME OVER! Press 'r' to restart or 'q' to quit")
            )?;
        } else {
            execute!(
                stdout(),
                MoveTo(0, (self.height + 3) as u16),
                SetForegroundColor(Color::White),
                Print("Controls: WASD or arrow keys | q - quit")
            )?;
        }

        stdout().flush()?;
        Ok(())
    }

    pub fn handle_input(&mut self) -> io::Result<bool> {
        if poll(Duration::from_millis(50))? {
            if let Event::Key(KeyEvent { code, .. }) = read()? {
                match code {
                    KeyCode::Char('q') | KeyCode::Char('Q') => return Ok(false),
                    KeyCode::Char('r') | KeyCode::Char('R') if self.game_over => {
                        *self = Game::new(self.width, self.height);
                    }
                    KeyCode::Char('w') | KeyCode::Char('W') | KeyCode::Up => {
                        self.snake.change_direction(Direction::Up);
                    }
                    KeyCode::Char('s') | KeyCode::Char('S') | KeyCode::Down => {
                        self.snake.change_direction(Direction::Down);
                    }
                    KeyCode::Char('a') | KeyCode::Char('A') | KeyCode::Left => {
                        self.snake.change_direction(Direction::Left);
                    }
                    KeyCode::Char('d') | KeyCode::Char('D') | KeyCode::Right => {
                        self.snake.change_direction(Direction::Right);
                    }
                    _ => {}
                }
            }
        }
        Ok(true)
    }
}
