use memoize::memoize;
use std::{
    collections::HashMap,
    io::{BufRead, BufReader},
};
#[allow(unused_imports)]
use tracing::{debug, event_enabled, info, Level};

pub type ResultType = u64;

#[derive(Debug, Default)]
pub struct Solution {
    devices: HashMap<String, Vec<String>>,
}
impl Solution {
    pub(crate) fn add_device(&mut self, source: String, targets: Vec<String>) {
        self.devices.insert(source, targets);
    }
}

#[allow(unused_variables, unused_mut)]
impl<T: std::io::Read> TryFrom<BufReader<T>> for Solution {
    type Error = std::io::Error;

    fn try_from(reader: BufReader<T>) -> Result<Self, Self::Error> {
        let mut solution = Self::default();
        for (id, line) in reader.lines().map_while(Result::ok).enumerate() {
            // Implement for problem
            let (source, targets) = line.split_once(':').unwrap();
            let source = source.trim().to_string();
            let targets = targets
                .split(' ')
                .filter(|s| !s.is_empty())
                .map(|s| s.to_string())
                .collect::<Vec<_>>();
            solution.add_device(source, targets);
        }
        Ok(solution)
    }
}
impl utils::Solution for Solution {
    type Result = anyhow::Result<ResultType>;
    fn analyse(&mut self, _is_full: bool) {}

    fn answer_part1(&self, is_full: bool) -> Self::Result {
        let r = if is_full {
            num_paths_part1(&self.devices, "you", "out")
        } else {
            0
        };
        // Implement for problem
        Ok(r as ResultType)
    }

    fn answer_part2(&self, is_full: bool) -> Self::Result {
        let r = num_paths_part2(&self.devices, is_full, "svr".to_string(), false, false);
        // Implement for problem
        Ok(r as ResultType)
    }
}

fn num_paths_part1(devices: &HashMap<String, Vec<String>>, current: &str, target: &str) -> usize {
    if current == target {
        1
    } else {
        match devices.get(current) {
            None => 0,
            Some(targets) => {
                let mut count = 0;
                for next in targets {
                    count += num_paths_part1(devices, next, target);
                }
                count
            }
        }
    }
}

#[memoize(Ignore:devices)]
fn num_paths_part2(
    devices: &HashMap<String, Vec<String>>,
    is_full: bool,
    current: String,
    seen_dac: bool,
    seen_fft: bool,
) -> usize {
    if current == "out" {
        if seen_dac && seen_fft {
            1
        } else {
            0
        }
    } else {
        match devices.get(&current) {
            None => 0,
            Some(targets) => {
                let mut count = 0;
                for next in targets {
                    count += num_paths_part2(
                        devices,
                        is_full,
                        next.to_owned(),
                        seen_dac || current == "dac",
                        seen_fft || current == "fft",
                    );
                }
                count
            }
        }
    }
}
