use crate::direction::Direction;
use crate::point::Point;
use std::collections::VecDeque;

pub struct Snake {
    pub body: VecDeque<Point>,
    pub direction: Direction,
}

impl Snake {
    pub fn new(x: i32, y: i32) -> Self {
        let mut body = VecDeque::new();
        body.push_back(Point { x, y });
        Snake {
            body,
            direction: Direction::Right,
        }
    }

    pub fn move_forward(&mut self) {
        let head = self.body.front().unwrap();
        let new_head = match self.direction {
            Direction::Up => Point {
                x: head.x,
                y: head.y - 1,
            },
            Direction::Down => Point {
                x: head.x,
                y: head.y + 1,
            },
            Direction::Left => Point {
                x: head.x - 1,
                y: head.y,
            },
            Direction::Right => Point {
                x: head.x + 1,
                y: head.y,
            },
        };
        self.body.push_front(new_head);
    }

    pub fn change_direction(&mut self, new_direction: Direction) {
        let opposite = match self.direction {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        };
        if new_direction != opposite {
            self.direction = new_direction;
        }
    }

    pub fn check_collision(&self, width: i32, height: i32) -> bool {
        let head = self.body.front().unwrap();

        if head.x < 0 || head.x >= width || head.y < 0 || head.y >= height {
            return true;
        }

        for (i, segment) in self.body.iter().enumerate() {
            if i > 0 && segment == head {
                return true;
            }
        }

        false
    }

    pub fn eat_food(&mut self, food: Point) -> bool {
        if self.body.front().unwrap() == &food {
            true
        } else {
            self.body.pop_back();
            false
        }
    }
}
