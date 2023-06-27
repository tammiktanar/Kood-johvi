mod car;
mod grid;

use std::time::{Instant};

use ggez::{Context, GameError, GameResult, graphics};
use ggez::event::EventHandler;
use ggez::glam::{Vec2};
use ggez::graphics::{Color, Image, Text};
use ggez::input::keyboard::KeyInput;
use ggez::winit::event::VirtualKeyCode;
use crate::game::car::{Car, CarDirection};
use crate::game::grid::{IntersectionGrid};

pub struct Game {
    assets: Assets,

    grid: IntersectionGrid,
    cell_size: u32,

    cars: Vec<Car>,

    top_speed: u32,
    min_speed: u32,

    cars_amount: u32,

    max_time: f32,
    min_time: f32,

    close_calls: u32,

    paused: bool,
    dev: bool,
}

impl Game {
    pub fn new(ctx: &mut Context) -> Game {
        // Load/create resources such as images here.
        Game {
            assets: Assets::new(ctx),

            grid: IntersectionGrid::new(14),
            cell_size: 72,

            cars: vec![],

            top_speed: 0,
            min_speed: 0,

            cars_amount: 0,

            max_time: 0.0,
            min_time: 0.0,
            close_calls: 0,

            paused: false,
            dev: false,
        }
    }
}

impl Game {
    fn add_car(&mut self, ctx: &mut Context, mut car: Car) {
        if self.grid.add_car(ctx, &mut car).is_some() {
            self.cars.push(car)
        }
    }

    pub fn update_stats(&mut self, i: usize) {
        if self.min_time == 0.0 {
            self.min_time = self.cars[i].created_at.elapsed().as_secs_f32();
        }

        if self.cars[i].created_at.elapsed().as_secs_f32() > self.max_time {
            self.max_time = self.cars[i].created_at.elapsed().as_secs_f32();
        } else if self.cars[i].created_at.elapsed().as_secs_f32() < self.min_time {
            self.min_time = self.cars[i].created_at.elapsed().as_secs_f32();
        }


        if self.min_speed == 0 {
            self.min_speed = self.cars[i].min_speed_reached as u32;
        }

        if self.cars[i].top_speed_reached > (self.top_speed as f32) {
            self.top_speed = self.cars[i].top_speed_reached as u32;
        } else if self.cars[i].min_speed_reached < (self.min_speed as f32) {
            self.min_speed = self.cars[i].min_speed_reached as u32;
        }
    }
}


impl EventHandler for Game {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        for i in (0..(self.cars.len())).rev() {
            if !self.cars[i].update(ctx, &mut self.grid) {
                self.cars_amount += 1;
                self.update_stats(i);

                self.cars.remove(i);
            }
        }

        //self.cars.retain_mut(|car| car.update(ctx, &mut self.grid));

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::WHITE);

        // Draw background
        canvas.draw(&self.assets.background, graphics::DrawParam::new());

        let cell_size = self.cell_size as f32;

        // Draw rectangles to show locked cells
        if self.dev {
            let rect = graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::Fill(graphics::FillOptions::default()),
                graphics::Rect::new(0.0, 0.0, cell_size, cell_size),
                Color::RED,
            )?;

            self.grid.iter()
                .for_each(|(pos, cell)| {
                    let time = ctx.time.time_since_start();
                    match cell.peek() {
                        None => return,
                        Some(lock) if lock.enter > time || lock.leave < time => return,
                        _ => {}
                    };

                    canvas.draw(&rect, graphics::DrawParam::new()
                        .dest(Vec2::new(pos.x as f32, pos.y as f32) * cell_size),
                    );
                });
        }
        // Draw cars
        self.cars.iter().for_each(|car| car.draw(&mut canvas));


        if self.paused {
            let cars_in_intersection = Text::new(format!("Cars that have entered the intersection: {}", self.cars_amount));


            let cars_max_time = Text::new(format!("Cars maximum pass through time: {} seconds", self.max_time));
            let cars_min_time = Text::new(format!("Cars minimum pass through time: {} seconds", self.min_time));

            let cars_top_speed = Text::new(format!("Cars top speed: {}km/h", self.top_speed));
            let cars_min_speed = Text::new(format!("Cars min speed: {}km/h", self.min_speed));

            let cars_close_calls = Text::new(format!("Cars close calls: {}", self.close_calls));
            let cars_collision = Text::new(format!("Cars collisions: {}", self.close_calls));


            canvas.draw(&cars_in_intersection, graphics::DrawParam::new().dest(Vec2::new(10.0, 10.0)));
            canvas.draw(&cars_max_time, graphics::DrawParam::new().dest(Vec2::new(10.0, 25.0)));
            canvas.draw(&cars_min_time, graphics::DrawParam::new().dest(Vec2::new(10.0, 40.0)));
            canvas.draw(&cars_top_speed, graphics::DrawParam::new().dest(Vec2::new(10.0, 55.0)));
            canvas.draw(&cars_min_speed, graphics::DrawParam::new().dest(Vec2::new(10.0, 70.0)));
            canvas.draw(&cars_close_calls, graphics::DrawParam::new().dest(Vec2::new(10.0, 85.0)));
            canvas.draw(&cars_collision, graphics::DrawParam::new().dest(Vec2::new(10.0, 100.0)));
        }


        canvas.finish(ctx)
    }

    fn key_down_event(&mut self, ctx: &mut Context, input: KeyInput, repeated: bool) -> Result<(), GameError> {
        if repeated { return Ok(()); }

        let code = match input {
            KeyInput { keycode: Some(code), .. } => code,
            _ => return Ok(())
        };

        match code {
            VirtualKeyCode::Up | VirtualKeyCode::W => {
                let start = CarDirection::Up;
                let end = start.random_matching();

                self.add_car(ctx, Car::new(start, end, start.get_color(&self.assets), Instant::now()))
            }
            VirtualKeyCode::Down | VirtualKeyCode::S => {
                let start = CarDirection::Down;
                let end = start.random_matching();

                self.add_car(ctx, Car::new(start, end, start.get_color(&self.assets), Instant::now()))
            }
            VirtualKeyCode::Left | VirtualKeyCode::A => {
                let start = CarDirection::Left;
                let end = start.random_matching();

                self.add_car(ctx, Car::new(start, end, start.get_color(&self.assets), Instant::now()))
            }
            VirtualKeyCode::Right | VirtualKeyCode::D => {
                let start = CarDirection::Right;
                let end = start.random_matching();

                self.add_car(ctx, Car::new(start, end, start.get_color(&self.assets), Instant::now()))
            }

            VirtualKeyCode::R => {
                let start = CarDirection::random();
                let end = start.random_matching();

                self.add_car(ctx, Car::new(start, end, start.get_color(&self.assets), Instant::now()))
            }

            VirtualKeyCode::Insert => {
                self.dev = !self.dev;
                // ctx.quit_requested = true
            }

            VirtualKeyCode::Escape => {
                self.paused = !self.paused;
                // ctx.quit_requested = true
            }

            _ => {}
        }

        Ok(())
    }
}


pub struct Assets {
    background: Image,

    _car_black: Image,
    car_blue: Image,
    car_green: Image,
    car_red: Image,
    car_yellow: Image,
}

impl Assets {
    pub fn new(ctx: &mut Context) -> Self {
        Self {
            background: Image::from_path(ctx, "/background.png").unwrap(),

            _car_black: Image::from_path(ctx, "/car_black.png").unwrap(),
            car_blue: Image::from_path(ctx, "/car_blue.png").unwrap(),
            car_green: Image::from_path(ctx, "/car_green.png").unwrap(),
            car_red: Image::from_path(ctx, "/car_red.png").unwrap(),
            car_yellow: Image::from_path(ctx, "/car_yellow.png").unwrap(),
        }
    }
}