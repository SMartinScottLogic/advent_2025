use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::Debug,
    io::{BufRead, BufReader},
};
#[allow(unused_imports)]
use tracing::{debug, event_enabled, info, Level};
use utils::grid::Matrix;

pub type ResultType = u64;

#[derive(Debug, Default)]
pub struct Solution {
    manifold: Matrix<char>,
}
impl Solution {
    pub fn set(&mut self, x: usize, y: usize, c: char) {
        self.manifold.set(x as isize, y as isize, c);
    }
}

#[allow(unused_variables, unused_mut)]
impl<T: std::io::Read> TryFrom<BufReader<T>> for Solution {
    type Error = std::io::Error;

    fn try_from(reader: BufReader<T>) -> Result<Self, Self::Error> {
        let mut solution = Self::default();
        for (y, line) in reader.lines().map_while(Result::ok).enumerate() {
            for (x, c) in line.chars().enumerate() {
                solution.set(x, y, c);
            }
        }
        Ok(solution)
    }
}
impl utils::Solution for Solution {
    type Result = anyhow::Result<ResultType>;
    fn analyse(&mut self, _is_full: bool) {}

    fn answer_part1(&self, _is_full: bool) -> Self::Result {
        // Find start
        let (maxx, maxy) = self.manifold.dimensions();
        let mut start_x = -1;
        let mut start_y = -1;
        for y in 0..=maxy {
            for x in 0..=maxx {
                if let Some('S') = self.manifold.get(x, y) {
                    start_x = x;
                    start_y = y;
                }
            }
        }

        let num_splits = num_splits(&self.manifold, start_x, start_y);

        // Implement for problem
        Ok(num_splits as ResultType)
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
        // Implement for problem
        let (maxx, maxy) = self.manifold.dimensions();
        let mut start_x = -1;
        let mut start_y = -1;
        for y in 0..=maxy {
            for x in 0..=maxx {
                if let Some('S') = self.manifold.get(x, y) {
                    start_x = x;
                    start_y = y;
                }
            }
        }
        let r = num_timelines(&mut HashMap::new(), &self.manifold, start_x, start_y);
        Ok(r as ResultType)
    }
}

fn num_splits(manifold: &Matrix<char>, pos_x: isize, pos_y: isize) -> usize {
    let mut queue = VecDeque::new();
    let mut has_seen = HashSet::new();
    let mut splitters = HashSet::new();
    queue.push_back((pos_x, pos_y));
    while let Some((px, py)) = queue.pop_front() {
        match manifold.get(px, py) {
            Some('S') => queue.push_back((px, py + 1)),
            Some('.') => {
                if has_seen.contains(&(px, py + 1)) {
                    continue;
                }

                queue.push_back((px, py + 1));
                has_seen.insert((px, py + 1));
            }
            Some('^') => {
                if !has_seen.contains(&(px - 1, py + 1)) {
                    queue.push_back((px - 1, py + 1));
                }
                if !has_seen.contains(&(px + 1, py + 1)) {
                    queue.push_back((px + 1, py + 1));
                }

                splitters.insert((px, py));
            }
            None => {}
            Some(c) => panic!("Some({})", c),
        }
    }
    debug!(num_splits = debug(splitters.len()));
    splitters.len()
}

fn num_timelines(
    memo: &mut HashMap<(isize, isize), isize>,
    manifold: &Matrix<char>,
    pos_x: isize,
    pos_y: isize,
) -> isize {
    if let Some(v) = memo.get(&(pos_x, pos_y)) {
        return *v;
    }
    let count = match manifold.get(pos_x, pos_y) {
        Some('S') => num_timelines(memo, manifold, pos_x, pos_y + 1),
        Some('.') => num_timelines(memo, manifold, pos_x, pos_y + 1),
        Some('^') => {
            // Went left + right
            [pos_x - 1, pos_x + 1]
                .iter()
                .map(|&x| num_timelines(memo, manifold, x, pos_y + 1))
                .sum()
        }
        None => 1,
        Some(c) => panic!("unknown: {}", c),
    };
    memo.insert((pos_x, pos_y), count);
    count
}
