use std::{
    collections::HashSet,
    io::{BufRead, BufReader},
};
#[allow(unused_imports)]
use tracing::{debug, event_enabled, info, Level};

pub type ResultType = u64;

#[derive(Debug, Default)]
pub struct Solution {
    part1_lines: Vec<Vec<String>>,
    part2_lines: Vec<String>,
}
impl Solution {
    pub fn add_line(&mut self, part1_line: Vec<String>, part2_line: String) {
        self.part1_lines.push(part1_line);
        self.part2_lines.push(part2_line);
    }
}

#[allow(unused_variables, unused_mut)]
impl<T: std::io::Read> TryFrom<BufReader<T>> for Solution {
    type Error = std::io::Error;

    fn try_from(reader: BufReader<T>) -> Result<Self, Self::Error> {
        let r = regex::Regex::new(r"\s+").unwrap();
        let mut solution = Self::default();
        for (id, line) in reader.lines().map_while(Result::ok).enumerate() {
            let p = r
                .split(line.trim())
                .map(|s| s.to_string())
                .collect::<Vec<_>>();
            solution.add_line(p, line.to_string());
        }
        Ok(solution)
    }
}
impl utils::Solution for Solution {
    type Result = anyhow::Result<ResultType>;
    fn analyse(&mut self, _is_full: bool) {
        assert_eq!(
            1,
            self.part1_lines
                .iter()
                .map(|line| line.len())
                .collect::<HashSet<_>>()
                .len()
        );
        assert_eq!(
            1,
            self.part2_lines
                .iter()
                .map(|line| line.chars().count())
                .collect::<HashSet<_>>()
                .len()
        )
    }

    fn answer_part1(&self, _is_full: bool) -> Self::Result {
        let mut lines = self.part1_lines.clone();
        let mut total = lines[0]
            .iter()
            .map(|v| v.parse::<ResultType>().unwrap())
            .collect::<Vec<_>>();
        let operations = lines.pop().unwrap();
        for line in lines.iter().skip(1) {
            for (i, operation) in operations.iter().enumerate() {
                let val = line[i].parse::<ResultType>().unwrap();
                if i == 1 {
                    info!("{} {} {}", val, total[i], i);
                }
                match operation.as_str() {
                    "+" => total[i] += val,
                    "*" => total[i] *= val,
                    _ => panic!("Unknown operation {}", operation),
                };
            }
        }
        info!(?total);
        let total = total.iter().sum();
        // Implement for problem
        Ok(total)
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
        let mut lines = self.part2_lines.clone();
        let operators = lines.pop().unwrap();
        let mut vals = Vec::new();
        let mut total = 0;
        for i in (0..operators.chars().count()).rev() {
            let val = lines
                .iter()
                .map(|line| line.chars().nth(i).unwrap())
                .filter(|c| *c != ' ')
                .collect::<String>();
            if val.is_empty() {
                continue;
            }
            debug!(val);
            let val = val.parse::<ResultType>().unwrap();
            vals.push(val);
            match operators.chars().nth(i).unwrap() {
                ' ' => {}
                '+' => {
                    total += vals.iter().fold(0, |mut acc, val| {
                        acc += val;
                        acc
                    });
                    vals.clear();
                }
                '*' => {
                    total += vals.iter().fold(1, |mut acc, val| {
                        acc *= val;
                        acc
                    });
                    vals.clear();
                }
                _ => panic!(),
            }
        }
        // Implement for problem
        Ok(total as ResultType)
    }
}
