extern crate core;

use std::cmp::Reverse;
use std::collections::{HashMap};
use std::error::Error;
use log::{LevelFilter};
use glam::{IVec2, UVec2};
use priority_queue::PriorityQueue;
use lib::{Board, Cell, Piece, PlaceInput, Robot, setup_logger};

fn main() -> Result<(), Box<dyn Error>> {
    setup_logger(LevelFilter::Info);

    let mut bot = MyBot::new();
    bot.run();

    Ok(())
}

struct MyBot {
    my_base: UVec2,
    enemy_base: UVec2,
}

impl MyBot {
    fn new() -> Self {
        Self {
            my_base: UVec2::ZERO,
            enemy_base: UVec2::ZERO,
        }
    }
}

impl Robot for MyBot {
    fn place(&mut self, input: &PlaceInput) -> Option<UVec2> {
        let score_board = get_score_board(input);
        let options = get_options(input);

        place_by_score_board(&options, input.piece, &score_board)
    }

    fn base_locations(&mut self, me: UVec2, enemy: UVec2) {
        self.my_base = me;
        self.enemy_base = enemy;
    }
}

//<editor-fold desc="FILTERS">
fn get_options(input: &PlaceInput) -> Vec<UVec2> {
    input.board.iter()
        .map(|(pos, _)| pos)
        .filter(|pos| pos.x + input.piece.size().x < input.board.size().x)
        .filter(|pos| pos.y + input.piece.size().y < input.board.size().y)
        .filter(|pos| validate_piece(input.board, input.piece, *pos))
        .collect()
}

fn get_score_board(input: &PlaceInput) -> Vec<Vec<i32>> {
    let mut score_board = Vec::new();
    (0..input.board.size().y)
        .for_each(|_| score_board.push(vec![0_i32; input.board.size().x as usize]));

    let enemy_iter = input.board.iter()
        .filter(|(_, cell)| matches!(cell, Cell::Enemy(_)))
        .map(|(pos, _)| pos);

    let range = 100;
    flood_fill(enemy_iter, range, input.board.size(), |pos| matches!(input.board.get(*pos).unwrap(), Cell::Empty))
        .for_each(|(pos, depth)| score_board[pos.y as usize][pos.x as usize] += range as i32 - depth as i32);

    let my_iter = input.board.iter()
        .filter(|(_, cell)| matches!(cell, Cell::Me(_)))
        .map(|(pos, _)| pos);

    let range = 10;
    flood_fill(my_iter, range, input.board.size(), |pos| matches!(input.board.get(*pos).unwrap(), Cell::Empty))
        .for_each(|(pos, depth)| {
            let value = &mut score_board[pos.y as usize][pos.x as usize];
            if *value > 0 {
                *value -= (range as i32 - depth as i32) * 2;
            } else {
                *value -= depth as i32 + range as i32 * 2;
            }
        });

    score_board
}

// fn order_by_distance(options: &mut [UVec2], piece: &Piece, target: UVec2) {
//     let piece_center = piece.average();
//     options.sort_unstable_by_key(|pos| {
//         let pos = *pos + piece_center;
//         OrderedFloat((target.as_vec2() - pos.as_vec2()).length())
//     })
// }

//</editor-fold>

//<editor-fold desc="PLACERS">


// fn place_towards(options: &[UVec2], piece: &Piece, target: UVec2) -> Option<UVec2> {
//     let piece_center = piece.average();
//     options.iter()
//         .map(|pos| *pos + piece_center)
//         .min_by_key(|pos| OrderedFloat((target.as_vec2() - pos.as_vec2()).length()))
//         .map(|pos| pos - piece_center)
// }
//
// fn place_random(options: &[UVec2]) -> Option<UVec2> {
//     if options.is_empty() {
//         return None;
//     }
//
//     let i = rand::thread_rng().gen_range(0..options.len());
//     Some(options[i])
// }

fn place_by_score_board(options: &[UVec2], piece: &Piece, score_board: &[Vec<i32>]) -> Option<UVec2> {
    options.iter()
        .max_by_key(|&placement| {
            piece.clone().offset(*placement).iter()
                .map(|pos| score_board[pos.y as usize][pos.x as usize])
                .sum::<i32>()
        })
        .copied()
}
//</editor-fold>

//<editor-fold desc="TESTERS">
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
                }
                Cell::Enemy(_) => false,
            }
        }.then_some(())?;

        Some(())
    }).is_some() && self_collided
}

#[allow(dead_code)]
const OFFSETS_4: [IVec2; 4] = [
    IVec2::new(1, 0),
    IVec2::new(0, 1),
    IVec2::new(-1, 0),
    IVec2::new(0, -1),
];

#[allow(dead_code)]
const OFFSETS_8: [IVec2; 8] = [
    IVec2::new(1, 0),
    IVec2::new(1, 1),
    IVec2::new(0, 1),
    IVec2::new(-1, 1),
    IVec2::new(-1, 0),
    IVec2::new(-1, -1),
    IVec2::new(0, -1),
    IVec2::new(1, -1),
];

/// A pretty naive flood fill function. Supports filtering which cells to expand to using a closure.
fn flood_fill<I, F>(points: I, range: u32, board_size: UVec2, mut predicate: F) -> impl Iterator<Item=(UVec2, u32)>
where I: Iterator<Item=UVec2>,
      F: FnMut(&UVec2) -> bool
{
    let mut seen = HashMap::new();

    let points_iter = points
        .inspect(|pos| { seen.insert(*pos, 0); })
        .map(|pos| (pos, Reverse(0)));

    let mut queue: PriorityQueue<UVec2, Reverse<u32>> = PriorityQueue::from_iter(points_iter);

    while !queue.is_empty() {
        let (pos, depth) = queue.pop().unwrap();
        let depth = depth.0;

        if depth < range {
            OFFSETS_8.into_iter()
                .map(|offset| pos.as_ivec2() + offset)
                .filter(|pos| pos.x >= 0 && pos.x < board_size.x as i32 && pos.y >= 0 && pos.y < board_size.y as i32)
                .map(|pos| pos.as_uvec2())
                .filter(&mut predicate)
                .filter(|pos| seen.insert(*pos, depth + 1).is_none())
                .for_each(|pos| { queue.push(pos, Reverse(depth + 1)); });
        }
    }

    seen.into_iter()
}
//</editor-fold>
