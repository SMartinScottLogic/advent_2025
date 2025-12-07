use memoize::memoize;
use std::{
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

        let num_splits = num_splits_path(&self.manifold, start_x, start_y);
        // Implement for problem
        Ok(num_splits as ResultType)
    }

    fn answer_part2(&self, is_full: bool) -> Self::Result {
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
        let r = num_timelines(&self.manifold, is_full, start_x, start_y);
        Ok(r as ResultType)
    }
}

fn num_splits_path(manifold: &Matrix<char>, pos_x: isize, pos_y: isize) -> usize {
    pathfinding::directed::bfs::bfs_reach((pos_x, pos_y), |(x, y)| match manifold.get(*x, *y) {
        None => vec![],
        Some('S') => vec![(*x, y + 1)],
        Some('.') => vec![(*x, y + 1)],
        Some('^') => vec![(x - 1, y + 1), (x + 1, y + 1)],
        Some(c) => panic!("Some({})", c),
    })
    .filter(|(x, y)| manifold.get(*x, *y) == Some(&'^'))
    .count()
}

#[memoize(Ignore:manifold)]
fn num_timelines(manifold: &Matrix<char>, is_full: bool, pos_x: isize, pos_y: isize) -> isize {
    let count = match manifold.get(pos_x, pos_y) {
        Some('S') => num_timelines(manifold, is_full, pos_x, pos_y + 1),
        Some('.') => num_timelines(manifold, is_full, pos_x, pos_y + 1),
        Some('^') => {
            // Went left + right
            [pos_x - 1, pos_x + 1]
                .iter()
                .map(|&x| num_timelines(manifold, is_full, x, pos_y + 1))
                .sum()
        }
        None => 1,
        Some(c) => panic!("unknown: {}", c),
    };
    count
}
