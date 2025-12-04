use std::io::{BufRead, BufReader};
#[allow(unused_imports)]
use tracing::{debug, event_enabled, info, Level};
use utils::grid::Matrix;

pub type ResultType = u64;

#[derive(Debug, Default)]
pub struct Solution {
    matrix: Matrix<char>,
}
impl Solution {
    pub fn set_matrix(&mut self, x: usize, y: usize, c: char) {
        self.matrix.set(x as isize, y as isize, c);
    }
}

#[allow(unused_variables, unused_mut)]
impl<T: std::io::Read> TryFrom<BufReader<T>> for Solution {
    type Error = std::io::Error;

    fn try_from(reader: BufReader<T>) -> Result<Self, Self::Error> {
        let mut solution = Self::default();
        for (y, line) in reader.lines().map_while(Result::ok).enumerate() {
            // Implement for problem
            for (x, c) in line.trim().chars().enumerate() {
                solution.set_matrix(x, y, c);
            }
        }
        Ok(solution)
    }
}
impl utils::Solution for Solution {
    type Result = anyhow::Result<ResultType>;
    fn analyse(&mut self, _is_full: bool) {}

    fn answer_part1(&self, _is_full: bool) -> Self::Result {
        let count = get_removable(&self.matrix).len();
        // Implement for problem
        Ok(count as ResultType)
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
        let mut removed = 0;
        let mut matrix = self.matrix.clone();
        loop {
            let removable = get_removable(&matrix);
            if removable.is_empty() {
                break;
            }
            removed += removable.len();
            for (x, y) in removable {
                matrix.set(x, y, '.');
            }
        }
        // Implement for problem
        Ok(removed as ResultType)
    }
}

fn get_removable(matrix: &Matrix<char>) -> Vec<(isize, isize)> {
    let mut removable = Vec::new();
    for y in matrix.min_y()..=matrix.max_y() {
        for x in matrix.min_x()..=matrix.max_x() {
            if let Some('@') = matrix.get(x, y) {
                let mut neighbours = 0;
                for dy in -1..=1 {
                    for dx in -1..=1 {
                        if dx == 0 && dy == 0 {
                            continue;
                        }
                        if let Some('@') = matrix.get(x + dx, y + dy) {
                            neighbours += 1;
                        }
                    }
                }
                if neighbours < 4 {
                    removable.push((x, y));
                }
            }
        }
    }
    removable
}
