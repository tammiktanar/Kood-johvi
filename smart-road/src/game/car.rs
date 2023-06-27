use std::sync::atomic::{AtomicU32, Ordering};
use std::time::{Duration, Instant};
use ggez::glam::{IVec2, Vec2};
use ggez::{Context, graphics};
use ggez::graphics::{Image};
use rand::Rng;
use crate::game::Assets;
use crate::game::grid::IntersectionGrid;

#[derive(Debug)]
pub struct Car {
    pub id: u32,

    pub grid_pos: IVec2,
    pub cell_traversal: f32,
    pub direction: CarDirection,

    pub max_speed: f32,

    pub turn: Option<Turn>,

    pub image: Option<Image>,

    pub created_at: Instant,

    pub top_speed_reached: f32,
    pub min_speed_reached: f32,

    pub fraction_of_top_speed: f32,
}

static CAR_ID_COUNTER: AtomicU32 = AtomicU32::new(0);

impl Car {
    pub fn new(start: CarDirection, end: CarDirection, image: Image, creation_time: Instant) -> Self {
        let route = Route::new(start, end);

        Car {
            id: CAR_ID_COUNTER.fetch_add(1, Ordering::SeqCst),

            grid_pos: route.start,
            direction: start,
            cell_traversal: 0.0,

            turn: route.turn,

            max_speed: get_speed(),
            image: Some(image),

            created_at: creation_time,
            top_speed_reached: 0.0,
            min_speed_reached: 0.0,

            fraction_of_top_speed: 100.0,
        }
    }

    pub fn update(&mut self, ctx: &mut Context, grid: &mut IntersectionGrid) -> bool {
        loop {
            if grid.is_out_of_bounds(self.grid_pos) {
                return false;
            }

            let lock = grid.get_cell(self.grid_pos).peek().unwrap();

            let lock_time = lock.leave - lock.middle;

            let time = ctx.time.time_since_start();
            // let time = Duration::from_secs_f32(ctx.time.ticks() as f32 * (1.0 / 60.0));
            let time_to_leave = lock.leave.saturating_sub(time);

            self.cell_traversal = 1.0 - time_to_leave.as_secs_f32() / lock_time.as_secs_f32();

            if let Some(turn) = &self.turn {
                if self.direction != turn.direction
                    && self.grid_pos == turn.pos
                // && self.cell_traversal >= 0.5
                {
                    self.direction = turn.direction
                }
            }

            if time_to_leave == Duration::ZERO {
                grid.get_cell_mut(self.grid_pos).pop();
                self.grid_pos = self.into_iter().nth(1).unwrap();
                continue;
            }

            return true;
        }
    }

    pub fn draw(&self, canvas: &mut graphics::Canvas) {
        if self.image.is_none() {
            return;
        }

        let cell_size = 72.0;

        let cell_center = Vec2::new(
            self.grid_pos.x as f32 * cell_size + cell_size * 0.5,
            self.grid_pos.y as f32 * cell_size + cell_size * 0.5,
        );

        let nudge = self.cell_traversal * cell_size;

        let offset = match self.direction {
            CarDirection::Up => Vec2::new(0.0, -nudge),
            CarDirection::Down => Vec2::new(0.0, nudge),
            CarDirection::Left => Vec2::new(-nudge, 0.0),
            CarDirection::Right => Vec2::new(nudge, 0.0),
        };

        let car_pos = cell_center + offset;

        canvas.draw(self.image.as_ref().unwrap(), graphics::DrawParam::new()
            .offset(Vec2::new(0.5, 0.5))
            .rotation(self.direction.get_rotation())
            .dest(car_pos),
        );
    }

    pub fn calc_stat_speed(&mut self, time: f32) {
        let speed = 1.0 / time;
        let km_h = speed * 25.0;
        let fraction = speed / self.max_speed * 100.0;

        if self.min_speed_reached == 0.0 {
            self.min_speed_reached = km_h;
        }

        if self.top_speed_reached < km_h {
            self.top_speed_reached = km_h;
        }

        if self.min_speed_reached > km_h {
            self.min_speed_reached = km_h;
        }

        if fraction < self.fraction_of_top_speed {
            self.fraction_of_top_speed = fraction;
        }
    }
}


#[derive(Debug, Copy, Clone, PartialEq)]
pub enum CarDirection {
    Up,
    Down,
    Left,
    Right,
}

fn get_speed() -> f32 {
    let mut rng = rand::thread_rng();
    let y: i32 = rng.gen_range(1..=3);
    match y {
        1 => 6.0,
        2 => 4.0,
        3 => 2.0,
        _ => 0.0,
    }
}

impl CarDirection {
    fn get_rotation(&self) -> f32 {
        match self {
            Self::Up => 0.0,
            Self::Down => std::f32::consts::PI,
            Self::Left => std::f32::consts::PI * 1.5,
            Self::Right => std::f32::consts::PI * 0.5,
        }
    }

    pub fn random() -> Self {
        let number = rand::thread_rng().gen_range(0..4);
        match number {
            0 => Self::Up,
            1 => Self::Down,
            2 => Self::Left,
            3 => Self::Right,
            _ => unreachable!(),
        }
    }

    pub fn random_matching(self) -> Self {
        let start = self;

        loop {
            let end = CarDirection::random();

            if start != end.reverse() {
                break end;
            }
        }
    }

    pub fn reverse(self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }

    pub fn get_color(self, assets: &Assets) -> Image {
        match self {
            Self::Up => assets.car_red.clone(),
            Self::Down => assets.car_yellow.clone(),
            Self::Left => assets.car_blue.clone(),
            Self::Right => assets.car_green.clone(),
        }
    }
}

impl Into<IVec2> for CarDirection {
    fn into(self) -> IVec2 {
        match self {
            CarDirection::Up => IVec2::new(0, -1),
            CarDirection::Down => IVec2::new(0, 1),
            CarDirection::Left => IVec2::new(-1, 0),
            CarDirection::Right => IVec2::new(1, 0),
        }
    }
}


#[derive(Debug)]
pub struct Turn {
    pub pos: IVec2,
    pub direction: CarDirection,
}

#[derive(Debug)]
pub struct Route {
    start: IVec2,
    turn: Option<Turn>,
}

impl Route {
    pub fn new(start_direction: CarDirection, end_direction: CarDirection) -> Self {
        if start_direction == end_direction.reverse() {
            panic!("Can't do U-turns in Route::new()!")
        }

        match start_direction {
            CarDirection::Up => {
                match end_direction {
                    CarDirection::Up => Self {
                        start: IVec2::new(8, 13),
                        turn: None,
                    },
                    CarDirection::Down => unreachable!(),
                    CarDirection::Left => Self {
                        start: IVec2::new(7, 13),
                        turn: Some(Turn { pos: IVec2::new(7, 6), direction: end_direction }),
                    },
                    CarDirection::Right => Self {
                        start: IVec2::new(9, 13),
                        turn: Some(Turn { pos: IVec2::new(9, 9), direction: end_direction }),
                    },
                }
            }
            CarDirection::Down => {
                match end_direction {
                    CarDirection::Up => unreachable!(),
                    CarDirection::Down => Self {
                        start: IVec2::new(5, 0),
                        turn: None,
                    },
                    CarDirection::Left => Self {
                        start: IVec2::new(4, 0),
                        turn: Some(Turn { pos: IVec2::new(4, 4), direction: end_direction }),
                    },
                    CarDirection::Right => Self {
                        start: IVec2::new(6, 0),
                        turn: Some(Turn { pos: IVec2::new(6, 7), direction: end_direction }),
                    },
                }
            }
            CarDirection::Left => {
                match end_direction {
                    CarDirection::Up => Self {
                        start: IVec2::new(13, 4),
                        turn: Some(Turn { pos: IVec2::new(9, 4), direction: end_direction }),
                    },
                    CarDirection::Down => Self {
                        start: IVec2::new(13, 6),
                        turn: Some(Turn { pos: IVec2::new(6, 6), direction: end_direction }),
                    },
                    CarDirection::Left => Self {
                        start: IVec2::new(13, 5),
                        turn: None,
                    },
                    CarDirection::Right => unreachable!(),
                }
            }
            CarDirection::Right => {
                match end_direction {
                    CarDirection::Up => Self {
                        start: IVec2::new(0, 7),
                        turn: Some(Turn { pos: IVec2::new(7, 7), direction: end_direction }),
                    },
                    CarDirection::Down => Self {
                        start: IVec2::new(0, 9),
                        turn: Some(Turn { pos: IVec2::new(4, 9), direction: end_direction }),
                    },
                    CarDirection::Left => unreachable!(),
                    CarDirection::Right => Self {
                        start: IVec2::new(0, 8),
                        turn: None,
                    },
                }
            }
        }
    }
}

impl<'a> IntoIterator for &'a Car {
    type Item = IVec2;
    type IntoIter = Iter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        let step: IVec2 = self.direction.into();
        Iter {
            pos: self.grid_pos - step,
            direction: self.direction,
            car: self,
        }
    }
}

pub struct Iter<'a> {
    pos: IVec2,
    direction: CarDirection,
    car: &'a Car,
}

impl<'a> Iterator for Iter<'a> {
    type Item = IVec2;

    fn next(&mut self) -> Option<Self::Item> {
        let step: IVec2 = self.direction.into();
        self.pos += step;

        if let Some(turn) = &self.car.turn {
            if turn.pos == self.pos {
                self.direction = turn.direction;
            }
        }

        return Some(self.pos);
    }
}