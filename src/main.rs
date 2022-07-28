extern crate sdl2;

use rand::Rng;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;

const MAP_SIZE: u32 = 32;
const TILE_SIZE: u32 = 16;

#[derive(PartialEq)]
enum Direction {
    NotMoving,
    Left,
    Right,
    Up,
    Down,
}

#[derive(PartialEq)]
struct Position {
    x: i32,
    y: i32,
}

struct Snake {
    head: Position,
    parts: Vec<Position>,
    dir: Direction,
    apple: Position,
    score: u32,
}

impl Snake {
    fn new() -> Snake {
        Snake {
            head: Position { x: 0, y: 0 },
            parts: Vec::new(),
            dir: Direction::NotMoving,
            apple: Position {
                x: rand::thread_rng().gen_range(1, MAP_SIZE as i32 - 1),
                y: rand::thread_rng().gen_range(1, MAP_SIZE as i32 - 1),
            },
            score: 0,
        }
    }

    fn change_direction(&mut self, new_dir: Direction) {
        if !self.parts.is_empty() {
            match new_dir {
                Direction::Left => {
                    if self.dir == Direction::Right {
                        return;
                    }
                }
                Direction::Right => {
                    if self.dir == Direction::Left {
                        return;
                    }
                }
                Direction::Up => {
                    if self.dir == Direction::Down {
                        return;
                    }
                }
                Direction::Down => {
                    if self.dir == Direction::Up {
                        return;
                    }
                }
                _ => {}
            }
            self.dir = new_dir;
        } else {
            self.dir = new_dir;
        }
    }

    fn apple_gen(&mut self) {
        while self.parts.contains(&Position { ..self.apple }) || self.apple == self.head {
            self.apple.x = rand::thread_rng().gen_range(0, MAP_SIZE as i32 - 1);
            self.apple.y = rand::thread_rng().gen_range(0, MAP_SIZE as i32 - 1);
        }
    }

    fn reset(&mut self) {
        self.head = Position {x: 0, y: 0};
        self.parts.clear();
        self.dir = Direction::NotMoving;
        self.apple_gen();
        self.score = 0;
    }

    fn update(&mut self) -> Result<(), String> {
        let mut prev = Position { ..self.head };
        match self.dir {
            Direction::Left => {
                self.head.x -= 1;
            }
            Direction::Right => {
                self.head.x += 1;
            }
            Direction::Up => {
                self.head.y -= 1;
            }
            Direction::Down => {
                self.head.y += 1;
            }
            _ => {}
        }
        for part in self.parts.iter() {
            if part.x == self.head.x && part.y == self.head.y {
                self.reset();
                return Ok(());
            }
        }
        if self.head.x < 0
            || self.head.x > MAP_SIZE as i32
            || self.head.y < 0
            || self.head.y > MAP_SIZE as i32
        {
            self.reset();
            return Ok(());
        }
        if self.head == self.apple {
            self.score += 1;
            self.apple_gen();
            if self.parts.is_empty() {
                self.parts.push(Position {
                    x: self.head.x,
                    y: self.head.y,
                });
            } else {
                self.parts.push(Position {
                    x: self.parts[self.parts.len() - 1].x,
                    y: self.parts[self.parts.len() - 1].y,
                });
            }
        }
        if !self.parts.is_empty() {
            for part in self.parts.iter_mut() {
                std::mem::swap(part, &mut prev);
            }
        }

        Ok(())
    }

    fn render(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        let head = sdl2::rect::Rect::new(
            self.head.x * TILE_SIZE as i32,
            self.head.y * TILE_SIZE as i32,
            TILE_SIZE,
            TILE_SIZE,
        );
        match canvas.fill_rect(head) {
            Err(e) => println!("{:?}", e),
            _ => (),
        }
        for part in self.parts.iter() {
            let head = sdl2::rect::Rect::new(
                part.x * TILE_SIZE as i32,
                part.y * TILE_SIZE as i32,
                TILE_SIZE,
                TILE_SIZE,
            );
            match canvas.fill_rect(head) {
                Err(e) => println!("{:?}", e),
                _ => (),
            }
        }
        let apple_rect = sdl2::rect::Rect::new(
            self.apple.x * TILE_SIZE as i32,
            self.apple.y * TILE_SIZE as i32,
            TILE_SIZE,
            TILE_SIZE,
        );
        canvas.set_draw_color(Color::RGB(255, 0, 0));
        match canvas.fill_rect(apple_rect) {
            Err(e) => println!("{:?}", e),
            _ => (),
        }
    }
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("snake-rs", MAP_SIZE * TILE_SIZE, MAP_SIZE * TILE_SIZE)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut snake = Snake::new();
    'running: loop {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::W),
                    ..
                } => {
                    snake.change_direction(Direction::Up);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::A),
                    ..
                } => {
                    snake.change_direction(Direction::Left);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::S),
                    ..
                } => {
                    snake.change_direction(Direction::Down);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::D),
                    ..
                } => {
                    snake.change_direction(Direction::Right);
                }
                _ => {}
            }
        }
        match snake.update() {
            Err(_) => {
                break 'running;
            }
            _ => (),
        }
        match canvas.window_mut().set_title(&format!("snake-rs score: {}", snake.score)) {
            Err(_) => {
                break 'running;
            }
            _ => (),
        }
        snake.render(&mut canvas);
        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 10 + (snake.score / 4)));
    }
}
