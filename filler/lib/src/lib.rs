use std::fs::File;
use std::{env, io};
use std::fmt::{Debug, Formatter};
use std::io::{BufRead, Write};
use std::time::Instant;
use glam::{IVec2, UVec2};
use anyhow::{anyhow, bail, Context, Error, Result};
use itertools::Itertools;
use log::{debug, error, info, LevelFilter, trace};
use simplelog::{CombinedLogger, ConfigBuilder, WriteLogger};

pub fn setup_logger(level: LevelFilter) {
    if let Err(err) = setup_logger_internal(level) {
        println!("logger setup failed: {}", err);
        panic!("logger setup failed: {}", err);
    }
}

pub trait Robot: Sized {
    /// Takes in the board and a piece to place, and returns the coordinates of where to place it
    fn place(&mut self, input: &PlaceInput) -> Option<UVec2>;

    #[deprecated(note="use Robot::init() instead")]
    #[allow(unused_variables)]
    fn base_locations(&mut self, me: UVec2, enemy: UVec2) {}

    #[allow(unused_variables)]
    fn init(&mut self, info: &InitInfo) {}

    fn run(&mut self) {
        let res = run_internal(self);
        if let Err(err) = res {
            error!("fatal: {}", err.chain().map(|s| s.to_string()).join(": "))
        }
    }
}

pub struct PlaceInput<'a> {
    pub board: &'a Board,
    pub piece: &'a Piece,
}

pub struct InitInfo {
    pub playing_as: Team,
    pub board_size: UVec2,
    pub my_base: UVec2,
    pub enemy_base: UVec2,
}


pub struct Board {
    size: UVec2,
    vec: Vec<Cell>,
}

impl Board {
    pub fn iter(&self) -> impl Iterator<Item=(UVec2, Cell)> + '_ {
        return self.vec.iter()
            .copied()
            .enumerate()
            .map(|(i, cell)| (UVec2::new(i as u32 % self.size.x, i as u32 / self.size.x), cell));
    }

    /// Gets a cell from the board. Returns None if it's out of the board's bounds.
    ///
    /// The input position can be any type that can be turned into an UVec2, so any of these work:
    /// UVec::new(x, y)
    /// (x, y)
    /// [x, y]
    pub fn get<T: Into<UVec2>>(&self, pos: T) -> Option<Cell> {
        let pos = pos.into();
        if pos.y >= self.size.y || pos.x >= self.size.x {
            return None
        }

        Some(self.vec[(self.size.x * pos.y + pos.x) as usize])
    }

    pub fn get_ivec<T: Into<IVec2>>(&self, pos: T) -> Option<Cell> {
        let pos = pos.into();
        if pos.min_element() < 0 {
            return None
        }
        self.get(pos.as_uvec2())
    }

    pub fn get_unchecked<T: Into<UVec2>>(&self, pos: T) -> Cell {
        let pos = pos.into();
        self.vec[(self.size.x * pos.y + pos.x) as usize]
    }

    pub fn size(&self) -> UVec2 {
        self.size
    }

    pub fn vec(&self) -> &Vec<Cell> {
        &self.vec
    }

    pub fn my_last_points(&self) -> Vec<UVec2> {
        self.last_points_internal(|(_, cell)| matches!(cell, Cell::Me(true)))
    }

    pub fn my_last_piece(&self) -> Result<(Piece, UVec2)> {
        let vec = self.my_last_points();
        Piece::from(vec)
    }

    pub fn enemy_last_points(&self) -> Vec<UVec2> {
        self.last_points_internal(|(_, cell)| matches!(cell, Cell::Enemy(true)))
    }

    pub fn enemy_last_piece(&self) -> Result<(Piece, UVec2)> {
        let vec = self.enemy_last_points();
        Piece::from(vec)
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Cell {
    Empty,
    // Bools are true when part of the latest placed piece
    Me(bool),
    Enemy(bool),
}

impl Cell {
    pub fn same_team_as(self, other: Self) -> bool {
        match self {
            Self::Empty => other == Self::Empty,
            Self::Me(_) => {
                matches!(other, Self::Me(_))
            }
            Self::Enemy(_) => {
                matches!(other, Self::Enemy(_))
            }
        }
    }
}


#[derive(Clone, PartialEq, Eq)]
pub struct Piece {
    size: UVec2,
    /// Guaranteed to be non-empty and minimums are 0
    points: Vec<UVec2>,
}

impl Piece {
    pub fn new(points: Vec<UVec2>) -> Result<Self> {
        if points.is_empty() {
            bail!("vector is empty")
        }

        let min_x = points.iter().min_by_key(|&pos| pos.x).unwrap().x;
        let min_y = points.iter().min_by_key(|&pos| pos.y).unwrap().y;
        if !(min_x == 0 && min_y == 0) {
            bail!("vector doesn't have minimum x/y positions of 0")
        }

        let max_x = points.iter().max_by_key(|&pos| pos.x).unwrap().x;
        let max_y = points.iter().max_by_key(|&pos| pos.y).unwrap().y;
        let size = UVec2::new(max_x + 1, max_y + 1);

        Ok(Self {
            size,
            points,
        })
    }

    pub fn from(points: Vec<UVec2>) -> Result<(Self, UVec2)> {
        if points.is_empty() {
            bail!("vector is empty")
        }

        let min_x = points.iter().min_by_key(|&pos| pos.x).unwrap().x;
        let min_y = points.iter().min_by_key(|&pos| pos.y).unwrap().y;
        let offset = UVec2::new(min_x, min_y);

        let points: Vec<_> = points.into_iter().map(|pos| pos - offset).collect();

        let max_x = points.iter().max_by_key(|&pos| pos.x).unwrap().x;
        let max_y = points.iter().max_by_key(|&pos| pos.y).unwrap().y;
        let size = UVec2::new(max_x + 1, max_y + 1);

        let piece = Self {
            size,
            points,
        };

        Ok((piece, offset))
    }

    pub fn size(&self) -> UVec2 {
        self.size
    }

    /// How many cells does this piece cover
    #[allow(clippy::len_without_is_empty)] // Can't be empty
    pub fn len(&self) -> usize {
        self.points.len()
    }

    /// Iterates over each cell this piece covers
    pub fn iter(&self) -> impl Iterator<Item=UVec2> + '_ {
        self.points.iter().copied()
    }

    /// Iterates like [Self::iter()], but with an offset applied
    pub fn iter_offset(&self, offset: UVec2) -> impl Iterator<Item=UVec2> + '_ {
        self.iter().map(move |pos| pos + offset)
    }

    /// Returns the average location of this cell
    pub fn average(&self) -> UVec2 {
        let sum = self.iter().reduce(|a, b| a + b).unwrap();
        sum / self.points.len() as u32
    }

    pub fn offset(self, offset: UVec2) -> Vec<UVec2> {
        self.points.into_iter().map(|point| point + offset).collect()
    }
}


//////////////////////////////////////
// TRAIT IMPLEMENTATIONS START HERE //
//////////////////////////////////////

impl Debug for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let x_coords: String = (0..self.size.x)
            .map(|n| n % 10)
            .map(|n| n.to_string())
            .collect();
        write!(f, "  {}", x_coords)?;

        let mut count = 0;
        self.iter().try_for_each(|(pos, cell)| {
            let c = match cell {
                Cell::Empty => '.',
                Cell::Me(false) => '@',
                Cell::Me(true) => 'a',
                Cell::Enemy(false) => '$',
                Cell::Enemy(true) => 's',
            };

            if count <= pos.y {
                write!(f, "\n{:0>2}", pos.y % 100)?;
                count += 1;
            };

            write!(f, "{}", c)?;

            Ok(())
        })
    }
}

impl Debug for Piece {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut buffer = vec![b'.'; (self.size.y * self.size.x) as usize];

        self.iter().for_each(|pos| {
            buffer[(self.size.x * pos.y + pos.x) as usize] = b'O'
        });

        let mut first = true;
        buffer.chunks(self.size.x as usize)
            .try_for_each(|row| {
                if first {
                    first = false;
                } else {
                    writeln!(f)?;
                }
                write!(f, "{}", String::from_utf8_lossy(row))
            })
    }
}


//////////////////////////
// INTERNALS START HERE //
//////////////////////////

fn setup_logger_internal(level: LevelFilter) -> Result<()> {
    let config = ConfigBuilder::new()
        .set_thread_level(LevelFilter::Off)
        .set_target_level(LevelFilter::Off)
        .build();

    let mut file_path = env::current_exe().context("could not get executable path")?;
    file_path.set_extension("log");

    let log_file = File::create(file_path).context("could not create log file")?;

    CombinedLogger::init(vec![
        // TermLogger::new(level, config.clone(), TerminalMode::Stderr, ColorChoice::Auto),
        WriteLogger::new(level, config, log_file),
    ]).context("another logger already set")?;

    std::panic::set_hook(Box::new(|info| {
        error!("{}\n\nBACKTRACE:\n{:?}", info, backtrace::Backtrace::new());
    }));

    Ok(())
}

impl Board {
    fn new<T: Into<UVec2>>(size: T, vec: Vec<Cell>) -> Self {
        let size = size.into();
        assert_eq!(vec.len() as u32, size.y * size.x);
        Self {
            size,
            vec,
        }
    }

    fn last_points_internal<P>(&self, predicate: P) -> Vec<UVec2>
    where P: FnMut(&(UVec2, Cell)) -> bool
    {
        self.iter()
            .filter(predicate)
            .map(|(pos,_)| pos)
            .collect()
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Team {
    P1,
    P2,
}

impl TryFrom<char> for Team {
    type Error = Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'a' | '@' => Ok(Self::P1),
            's' | '$' => Ok(Self::P2),
            _ => Err(anyhow!("{} is not a valid team char", value)),
        }
    }
}


fn run_internal<T: Robot>(robot: &mut T) -> Result<()> {
    let mut lines = io::stdin().lock().lines();

    info!("robot loop starting");

    // $$$ exec p1 : [solution/megabot]
    let init = lines.next().unwrap().unwrap();
    let my_team = match &init[9..11] {
        "p1" => Team::P1,
        "p2" => Team::P2,
        _ => bail!("unknown player identifier"),
    };

    info!("playing as {:?}", my_team);

    // Used give robot info only once
    let mut init_done = false;

    // Reusable board
    let mut board = Board::new(UVec2::ZERO, vec![]);

    // Set to true when robot can't find a placement
    let mut ready_to_exit = false;

    let mut turn_counter = 1;

    loop {
        info!("-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-  TURN {}  -=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-", turn_counter);
        turn_counter += 1;

        // Anfield 20 15:
        let line = lines.next();
        let line = match line {
            None if ready_to_exit => {
                info!("no more pieces could be placed, exiting");
                return Ok(())
            }
            v => v.context("couldn't read board size line (last placement location was probably invalid)")??,
        };

        let board_size = parse_size(&line, 8).context("parsing board size")?;

        // 01234567890123456789 <- skip this
        lines.next();

        // Parse rows
        // 000 ..a@a.......s$s.....
        let row_count = board_size.y as usize;
        let board_lines = collect_lines(lines.by_ref().take(row_count), row_count)?;
        let cells = board_lines.iter()
            .flat_map(move |s| parse_cells_row(s, my_team));

        let mut vec = board.vec;
        vec.clear();
        vec.reserve_exact((board_size.y * board_size.x) as usize);
        vec.extend(cells);
        board = Board::new(board_size, vec);

        trace!("board:\n{:?}", board);

        // Piece 4 1:
        let line = lines.next().context("couldn't read piece size line")??;
        let piece_size = parse_size(&line, 6)?;

        // Parse piece
        // OOO.
        let row_count = piece_size.y as usize;
        let piece_lines = collect_lines(lines.by_ref().take(row_count), row_count)?;

        let piece_points = parse_piece(piece_lines);
        let (piece, offset) = Piece::from(piece_points)?;


        // Tell the robot where the bases are located
        if !init_done {
            let (my_base, _) = board.iter().find(|(_, cell)| cell.same_team_as(Cell::Me(false)))
                .context("couldn't find my base location")?;
            let (enemy_base, _) = board.iter().find(|(_, cell)| cell.same_team_as(Cell::Enemy(false)))
                .context("couldn't find my base location")?;

            trace!("my base: ({}, {}); Enemy base: ({}, {})", my_base.x, my_base.y, enemy_base.x, enemy_base.y);

            let init_info = InitInfo {
                playing_as: my_team,
                board_size,
                my_base,
                enemy_base,
            };
            robot.init(&init_info);
            #[allow(deprecated)]
            robot.base_locations(my_base, enemy_base);

            init_done = true;
        }

        trace!("piece with offset ({}, {}):\n{:?}", offset.x, offset.y, piece);

        // Make the robot place a piece
        debug!("starting placement");
        let time = Instant::now();
        let input = PlaceInput {
            board: &board,
            piece: &piece,
        };
        let placement = robot.place(&input).unwrap_or_else(|| {
            info!("robot didn't return a placement, defaulting to (0, 0)");
            ready_to_exit = true;
            UVec2::ZERO
        });

        let time = Instant::now() - time;
        debug!("got placement ({}, {}) with offset ({}, {})", placement.x, placement.y, offset.x, offset.y);
        let placement = placement.as_ivec2() - offset.as_ivec2();

        println!("{} {}", placement.x, placement.y);
        io::stdout().flush()?;
        info!("calculating finished in {:?}", time);
        info!("piece placed at: ({}, {})", placement.x, placement.y);
    }
}

fn collect_lines<T>(lines: T, capacity: usize) -> Result<Vec<String>> where
    T: Iterator<Item=std::result::Result<String, io::Error>>
{
    let mut result = Vec::with_capacity(capacity);
    for line in lines {
        result.push(line?);
    }
    Ok(result)
}

/// Parses lines like these:
/// Anfield 20 15:
/// Piece 3 2:
fn parse_size(line: &str, offset: usize) -> Result<UVec2> {
    let results: Vec<_> = line[offset..line.len() - 1]
        .split(' ')
        .filter_map(|str| str.parse::<u32>().ok())
        .collect();

    if results.len() != 2 {
        bail!("didn't find 2 sizes")
    }

    let unwrapped: Vec<_> = results.into_iter().collect();

    let size = UVec2::new(unwrapped[0], unwrapped[1]);

    Ok(size)
}

/// Parses lines like
/// 069 ..a@a.......s$s.....
fn parse_cells_row(line: &str, my_team: Team) -> impl Iterator<Item=Cell> + '_ {
    line.chars()
        .skip(4)
        .map(move |c| {
            match c {
                's' | '$' | 'a' | '@' => {
                    let team = Team::try_from(c).unwrap();
                    let latest = c == 's' || c == 'a';
                    match team == my_team {
                        true => Cell::Me(latest),
                        false => Cell::Enemy(latest),
                    }
                }
                _ => Cell::Empty,
            }
        })
}

/// Parses a block of lines like
/// .OOO.
/// .O...
fn parse_piece(lines: Vec<String>) -> Vec<UVec2> {
    lines.iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.chars()
                .enumerate()
                .map(move |(x, c)| (UVec2::new(x as u32, y as u32), c))
        })
        .filter_map(|(pos, c)| (c == 'O').then_some(pos))
        .collect()
}
