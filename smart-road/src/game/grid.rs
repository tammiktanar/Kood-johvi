use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::time::Duration;
use ggez::Context;
use ggez::glam::IVec2;
use crate::game::car::Car;

pub struct IntersectionGrid {
    side_len: u32,

    pub vec: Vec<Vec<GridCell>>,
}

impl IntersectionGrid {
    pub fn new(side_len: u32) -> Self {
        IntersectionGrid {
            side_len,

            // Create a (side_len x side_len) 2D vector
            vec: (0..side_len).map(|_y| {
                (0..side_len).map(|_x| {
                    GridCell::new()
                }).collect()
            }).collect(),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (IVec2, &GridCell)> {
        self.vec.iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .map(move |(x, cell)| (IVec2::new(x as i32, y as i32), cell))
            })
    }

    pub fn is_out_of_bounds(&self, pos: IVec2) -> bool {
        pos.x < 0 || pos.y < 0 || pos.x >= self.side_len as i32 || pos.y >= self.side_len as i32
    }

    pub fn get_cell(&self, pos: IVec2) -> &GridCell {
        &self.vec[pos.y as usize][pos.x as usize]
    }

    pub fn get_cell_mut(&mut self, pos: IVec2) -> &mut GridCell {
        &mut self.vec[pos.y as usize][pos.x as usize]
    }

    pub fn reset(&mut self) {
        self.vec.iter_mut().for_each(|row| {
            row.iter_mut().for_each(|cell| {
                cell.reset()
            })
        })
    }

    pub fn add_car(&mut self, ctx: &mut Context, car: &mut Car) -> Option<()> {
        let time = ctx.time.time_since_start();
        // let time = Duration::from_secs_f32(ctx.time.ticks() as f32 * (1.0 / 60.0));

        self.add_car_inner(time, car)
    }

    fn add_car_inner(&mut self, time: Duration, car: &mut Car) -> Option<()> {
        let positions: Vec<_> = car.into_iter()
            .take_while(|&pos| !self.is_out_of_bounds(pos))
            .collect();

        // Function can bail here when not enough room to spawn a car
        let keyframes = self.get_keyframes(time, car, &positions)?;

        println!("Adding car: {:<5} -> {:?}", format!("{:?}", car.direction), car.turn.as_ref().map(|t| t.direction).unwrap_or(car.direction));

        let mut time = time;
        let mut prev_i = 0;

        // Generate cell enter times from keyframes
        let mut lock_starts = Vec::with_capacity(positions.len());
        lock_starts.push(time);

        for (i, k_time) in keyframes.iter().copied() {
            let step_time = (k_time - time) / (i - prev_i) as u32;

            for _ in prev_i..i {
                time += step_time;
                lock_starts.push(time);
            }

            time = k_time;
            prev_i = i;
        }

        // Infer leave times from enter times list (last 2 not possible)
        let mut locks: Vec<_> = lock_starts.windows(3)
            .map(|slc| {
                let [a, b, c]: [_; 3] = slc.try_into().unwrap();

                CellLock::new(a, b, c)
            })
            .collect();

        // Infer missing lock based on last lock
        let last = locks.last().unwrap();
        locks.push(CellLock::new(last.middle, last.leave, last.leave + (last.leave - last.middle)));

        // Apply locks
        assert_eq!(locks.len(), positions.len(), "Amount of locks not equal to positions!");

        for cell_lock in locks.iter() {
            car.calc_stat_speed( (cell_lock.leave - cell_lock.middle).as_secs_f32() as f32);
        }

        locks.into_iter().zip(positions.into_iter())
            .for_each(|(lock, pos)| {
                self.get_cell_mut(pos).locks.push(lock);
            });


        Some(())
    }

    fn get_keyframes(&mut self, time: Duration, car: &Car, positions: &[IVec2]) -> Option<Vec<(usize, Duration)>> {
        let min_step_time = Duration::from_secs_f32(1.0 / car.max_speed);

        self.lock_controller(positions, min_step_time, 0, time, time)
            .map(|keyframes| keyframes.into_iter().rev().collect::<Vec<_>>())
            .ok()
    }


    const MARGIN: Duration = Duration::from_millis(1);

    /// Sends crawlers to determine how fast and how far to go in a given section
    fn lock_controller(&self, positions: &[IVec2], min_step_time: Duration, i: usize, section_start: Duration, prev_start: Duration) -> Result<Vec<(usize, Duration)>, Duration> {
        // println!("Section start: {}, {:?}", i, section_start);
        let mut step_time = min_step_time;
        let mut max_i = positions.len();


        loop {
            let res = self.lock_crawler(positions, step_time, i, max_i, section_start, prev_start);

            match res {
                Ok((last_enter, next_enter)) => {
                    // Crawler was able to reach the destination without collisions
                    if max_i == positions.len() {
                        // Yay, we reached the end!
                        return Ok(vec![(max_i, next_enter)]);
                    } else {
                        // Segment finished, but more to go
                        let recursion = self.lock_controller(positions, min_step_time, max_i, next_enter, last_enter);
                        match recursion {
                            Ok(mut keyframes) => {
                                // Recursion child reached the end!
                                keyframes.push((max_i, next_enter));
                                return Ok(keyframes);
                            }
                            Err(collision_free_at) => {
                                // Recursion child collided at section border
                                let duration = collision_free_at - section_start;
                                step_time = duration / (max_i - i) as u32;
                                continue;
                            }
                        }
                    }
                }
                Err((collision_i, collision_free_at)) => {
                    // Crawler had a collision!
                    if collision_i <= i {
                        // Collision at section border, previous section needs to use a different speed.
                        // println!("Section not viable: {:?}", collision_free_at);
                        return Err(collision_free_at);
                    }

                    // Compensating speed based on colliding lock
                    let section_duration = collision_free_at - section_start;
                    let section_steps = collision_i - i;

                    step_time = section_duration / section_steps as u32;
                    max_i = collision_i;

                    // println!("pos {} - {} collided: {:?} - ({:?} -> {:?}); Speed {}", i, collision_i, section_start, section_start + step_time * section_steps as u32, collision_free_at, 1.0 / step_time.as_secs_f32());
                    continue;
                }
            }
        }
    }

    /// Crawls forward up to max_i while checking for any collisions
    fn lock_crawler(&self,
                    positions: &[IVec2],
                    step_time: Duration,
                    i: usize,
                    max_i: usize,
                    current_enter: Duration,
                    prev_enter: Duration,
    ) -> Result<(Duration, Duration), (usize, Duration)>
    {
        let next_enter = current_enter + step_time;
        let prev_leave = next_enter;

        if let Some(prev_i) = i.checked_sub(1) {
            // Now we know the previous cell's leave time so we check if it collides.
            let prev_pos = positions[prev_i];

            self.verify_lock(prev_pos, CellLock::new(prev_enter, current_enter, prev_leave))
                .map_err(|prev_free_at| (prev_i, prev_free_at + Self::MARGIN))?
        }

        if i == max_i {
            return Ok((prev_enter, current_enter));
        }

        self.lock_crawler(positions, step_time, i + 1, max_i, next_enter, current_enter)
    }

    /// Check if a lock at a given position would overlap an existing one
    fn verify_lock(&self, pos: IVec2, lock: CellLock) -> Result<(), Duration> {
        let cell = &self.vec[pos.y as usize][pos.x as usize];

        if let Some(overlapping) = cell.locks.iter().find(|&other_lock| lock.overlaps(other_lock)) {
            return Err(overlapping.leave);
        }

        Ok(())
    }
}

pub struct GridCell {
    pub locks: BinaryHeap<CellLock>,
}

impl GridCell {
    fn new() -> Self {
        GridCell {
            locks: BinaryHeap::new()
        }
    }

    fn reset(&mut self) {
        self.locks.clear()
    }

    pub fn peek(&self) -> Option<&CellLock> {
        self.locks.peek()
    }

    pub fn pop(&mut self) -> Option<CellLock> {
        self.locks.pop()
    }

}

#[derive(Debug)]
pub struct CellLock {
    pub enter: Duration,
    pub middle: Duration,
    pub leave: Duration,
}

impl CellLock {
    fn new(enter: Duration, middle: Duration, leave: Duration) -> Self {
        Self {
            enter,
            middle,
            leave,
        }
    }

    fn overlaps(&self, other: &Self) -> bool {
        self.enter < other.leave && self.leave > other.enter
    }
}

impl PartialEq<Self> for CellLock {
    fn eq(&self, other: &Self) -> bool {
        self.enter == other.enter && self.leave == other.leave
    }
}

impl PartialOrd for CellLock {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.enter.partial_cmp(&other.enter).map(|order| order.reverse())
    }
}

impl Eq for CellLock {}

impl Ord for CellLock {
    fn cmp(&self, other: &Self) -> Ordering {
        self.enter.cmp(&other.enter).reverse()
    }
}
