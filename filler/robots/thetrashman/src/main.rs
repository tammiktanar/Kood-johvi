use std::ops::Add;

use log::{debug, error, info, LevelFilter, trace, warn};
use rand::Rng;
use glam::{IVec2, UVec2, uvec2};
use ordered_float::OrderedFloat;
use lib::{Board, Cell, Piece, Robot, setup_logger};
use rayon::prelude::*;
use lib::PlaceInput;

fn main() {
    setup_logger(LevelFilter::Debug);
    let mut bot = MyBot::new();
    bot.run();
}

struct MyBot {
    my_location: UVec2,
    enemy_location: UVec2,
}

impl MyBot {
    fn new() -> Self {
        Self {
            my_location: UVec2::new(0,0),
            enemy_location: UVec2::new(0,0),
        }
    }
}

impl Robot for MyBot {
    fn place(&mut self, input: &PlaceInput) -> Option<UVec2> {
        let board = input.board;
        let piece = input.piece;

        let enemy_last_piece = board.enemy_last_piece()
            .map(|(piece, offset)| {
                offset + piece.average()
            })
            .unwrap_or(self.enemy_location);

        let all_options: Vec<UVec2> = get_all_possibilities(board, piece);

        if all_options.is_empty() {
            return None
        }

        counter_enemy(&all_options, enemy_last_piece)
    }

    fn base_locations(&mut self, me: UVec2, enemy: UVec2) {
        self.enemy_location = enemy;
        self.my_location = me;
    }
}


fn get_all_possibilities(board: &Board, piece:&Piece) -> Vec<UVec2>{
    let options = board.iter().par_bridge()
        .filter(|(pos, _)| validate_piece(board, piece, *pos))
        .map(|(pos, _)| pos)
        .collect();

    return options
}

fn move_towards_enemy_base(bot: &mut MyBot, board: &Board, options: &Vec<UVec2>) -> Option<UVec2>{
    let enemy_base_to_the_left = bot.my_location.x < bot.enemy_location.x;
    let enemy_base_below = bot.my_location.y > bot.enemy_location.y;

    let on_x_axis = bot.my_location.x == bot.enemy_location.x;
    let on_y_axis = bot.my_location.y == bot.enemy_location.y;

    if on_x_axis {
        if enemy_base_below {
            place_up(options, board)
        } else {
            place_down(options, board)
        }
    } else if on_y_axis {
        if enemy_base_to_the_left {
            place_left(options, board)
        } else {
            place_right(options, board)
        }
    } else {
        //place_towards(options, board.size()) // Diagonal Bottom right
        //place_towards(options, UVec2::new(0, board.size().y)) // Diagonal Bottom left
        //place_towards(options, UVec2::new(0, 0)) // Diagonal Upper left
        //place_towards(options, UVec2::new(board.size().x, 0)) // Diagonal Upper right


        // counter

        //counter_enemy(&all_options, enemy_last_piece)

        place_towards(options, UVec2::new(bot.enemy_location.x, bot.enemy_location.y))
    }
}

fn place_towards(options: &Vec<UVec2>, towards: UVec2) -> Option<UVec2> {
    let key = options.into_iter().min_by_key(|position| {
        let difference = (towards.as_vec2() - position.as_vec2()).length();

        OrderedFloat(difference)
    });

    key.copied()
}


fn place_up(options: &Vec<UVec2>, board: &Board) -> Option<UVec2> {
    place_towards(options, UVec2::new((board.size().x/2) as u32, 0))
}

fn place_down(options: &Vec<UVec2>, board: &Board) -> Option<UVec2> {
    place_towards(options, UVec2::new((board.size().x/2) as u32, board.size().y))
}

fn place_left(options: &Vec<UVec2>, board: &Board) -> Option<UVec2> {
    place_towards(options, UVec2::new(0, (board.size().y/2) as u32))
}

fn place_right(options: &Vec<UVec2>, board: &Board) -> Option<UVec2> {
    place_towards(options, UVec2::new(board.size().x, (board.size().y/2) as u32))
}

fn counter_enemy(options: &Vec<UVec2>, enemy_location: UVec2) -> Option<UVec2> {
    place_towards(options, enemy_location)
}   

fn place_first(options: &Vec<UVec2>) -> Option<UVec2> {
    Some(options[0])
}

fn place_last(options: &Vec<UVec2>) -> Option<UVec2> {
    Some(options[options.len()-1])
}

fn place_random(options: &Vec<UVec2>) -> Option<UVec2> {
    let i = rand::thread_rng().gen_range(0..options.len());
    Some(options[i])
}

fn validate_piece(board: &Board, piece: &Piece, pos: UVec2) -> bool {
    let mut self_collided = false;
    piece.iter().try_for_each(|offset| {
        let cell = board.get(pos + offset)?;

        if self_collided {
            cell == Cell::Empty
        } else {
            match cell {
                Cell::Empty => true,
                Cell::Me(_) => {
                    self_collided = true;
                    true
                },
                Cell::Enemy(_) => false,
            }
        }.then_some(())?;

        Some(())
    }).is_some() && self_collided
}