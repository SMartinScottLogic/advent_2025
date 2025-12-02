use std::{
    collections::HashSet,
    io::{BufRead, BufReader},
};
#[allow(unused_imports)]
use tracing::{debug, event_enabled, info, Level};

pub type ResultType = u64;

#[derive(Debug, Default)]
pub struct Solution {
    ranges: Vec<(String, String)>,
}
impl Solution {
    pub fn add_range(&mut self, start: &str, end: &str) {
        self.ranges.push((start.to_owned(), end.to_owned()));
    }
}

#[allow(unused_variables, unused_mut)]
impl<T: std::io::Read> TryFrom<BufReader<T>> for Solution {
    type Error = std::io::Error;

    fn try_from(reader: BufReader<T>) -> Result<Self, Self::Error> {
        let mut solution = Self::default();
        for (id, line) in reader.lines().map_while(Result::ok).enumerate() {
            for p in line.trim().split(',') {
                let (start, end) = p.split_once('-').unwrap();
                solution.add_range(start, end);
            }
        }
        Ok(solution)
    }
}
impl utils::Solution for Solution {
    type Result = anyhow::Result<ResultType>;
    fn analyse(&mut self, _is_full: bool) {}

    fn answer_part1(&self, _is_full: bool) -> Self::Result {
        let mut total = 0;
        for (start, end) in &self.ranges {
            let start_val = start.parse::<u64>().unwrap();
            let end_val = end.parse::<u64>().unwrap();
            let mut s = start[..start.len() / 2].parse::<u64>().unwrap_or_default();
            loop {
                let probe = format!("{}{}", s, s).parse::<u64>().unwrap();
                if probe > end_val {
                    break;
                }
                if probe >= start_val && probe <= end_val {
                    total += probe;
                }
                s += 1;
            }
        }
        // Implement for problem
        Ok(total)
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
        let mut total = 0;
        for (start, end) in &self.ranges {
            let start_val = start.parse::<u64>().unwrap();
            let end_val = end.parse::<u64>().unwrap();
            let mut values = HashSet::new();
            for val in start_val..=end_val {
                let val_str = format!("{}", val);
                for repeat_len in 1..=end.len() / 2 {
                    for repeat in start.len() / repeat_len..=end.len() / repeat_len {
                        if repeat < 2 {
                            continue;
                        }
                        if val_str
                            == val_str
                                .chars()
                                .take(repeat_len)
                                .collect::<String>()
                                .repeat(repeat)
                        {
                            values.insert(val);
                        }
                    }
                }
            }
            for val in values {
                debug!("{}-{}: {}", start, end, val);
                total += val;
            }
        }
        // Implement for problem
        Ok(total)
    }
}
