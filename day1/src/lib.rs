use std::io::{BufRead, BufReader};
#[allow(unused_imports)]
use tracing::{debug, event_enabled, info, Level};

pub type ResultType = u64;

#[derive(Debug, Default)]
pub struct Solution {
    rotations: Vec<(char, isize)>,
    // Analysis results
    num_zeros: ResultType,
    num_end_zeros: ResultType,
}
impl Solution {
    pub fn add_rotation(&mut self, direction: char, count: isize) {
        self.rotations.push((direction, count));
    }
}

#[allow(unused_variables, unused_mut)]
impl<T: std::io::Read> TryFrom<BufReader<T>> for Solution {
    type Error = std::io::Error;

    fn try_from(reader: BufReader<T>) -> Result<Self, Self::Error> {
        let mut solution = Self::default();
        for (id, line) in reader.lines().map_while(Result::ok).enumerate() {
            let direction = line.trim().chars().next().unwrap();
            let count = line
                .trim()
                .chars()
                .skip(1)
                .collect::<String>()
                .parse()
                .unwrap();
            solution.add_rotation(direction, count);
        }
        Ok(solution)
    }
}
impl utils::Solution for Solution {
    type Result = anyhow::Result<ResultType>;
    fn analyse(&mut self, _is_full: bool) {
        self.num_zeros = 0;
        self.num_end_zeros = 0;
        let mut position = 50;
        info!(position);
        for (direction, count) in &self.rotations {
            let dir = match direction {
                'R' => 1,
                'L' => -1,
                _ => panic!(),
            };
            for _ in 1..=*count {
                position += dir;
                if position == 100 {
                    position = 0;
                }
                if position == -1 {
                    position = 99;
                }
                if position == 0 {
                    self.num_zeros += 1;
                }
            }
            if position == 0 {
                self.num_end_zeros += 1;
            }
        }
    }

    fn answer_part1(&self, _is_full: bool) -> Self::Result {
        Ok(self.num_end_zeros as ResultType)
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
        Ok(self.num_zeros as ResultType)
    }
}
