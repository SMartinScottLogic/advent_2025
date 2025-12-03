use std::{
    collections::HashMap,
    io::{BufRead, BufReader},
};
#[allow(unused_imports)]
use tracing::{debug, event_enabled, info, Level};

pub type ResultType = u64;

#[derive(Debug, Default)]
pub struct Solution {
    banks: Vec<String>,
}
impl Solution {
    pub fn add_bank(&mut self, bank: &str) {
        self.banks.push(bank.to_string());
    }
}

#[allow(unused_variables, unused_mut)]
impl<T: std::io::Read> TryFrom<BufReader<T>> for Solution {
    type Error = std::io::Error;

    fn try_from(reader: BufReader<T>) -> Result<Self, Self::Error> {
        let mut solution = Self::default();
        for (id, line) in reader.lines().map_while(Result::ok).enumerate() {
            solution.add_bank(line.trim());
            // Implement for problem
        }
        Ok(solution)
    }
}
impl utils::Solution for Solution {
    type Result = anyhow::Result<ResultType>;
    fn analyse(&mut self, _is_full: bool) {}

    fn answer_part1(&self, _is_full: bool) -> Self::Result {
        let r: i64 = self
            .banks
            .iter()
            .map(|bank| {
                let bank = bank
                    .chars()
                    .map(|c| c.to_digit(10).unwrap() as i64)
                    .collect::<Vec<_>>();
                let best = get_best(&bank, 2, &mut HashMap::new()).unwrap();
                debug!(bank = ?bank, best);
                best
            })
            .sum();
        Ok(r as ResultType)
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
        let r: i64 = self
            .banks
            .iter()
            .map(|bank| {
                let bank = bank
                    .chars()
                    .map(|c| c.to_digit(10).unwrap() as i64)
                    .collect::<Vec<_>>();
                get_best(&bank, 12, &mut HashMap::new()).unwrap()
            })
            .sum();
        Ok(r as ResultType)
    }
}

fn get_best(bank: &[i64], digits: usize, memo: &mut HashMap<(usize, usize), i64>) -> Option<i64> {
    let mut max = None;
    if bank.len() < digits {
        return max;
    }
    if memo.contains_key(&(bank.len(), digits)) {
        return Some(*memo.get(&(bank.len(), digits)).unwrap());
    }
    for i in 0..bank.len() {
        if digits == 1 {
            let score = bank[i];
            max = match max {
                Some(v) => Some(std::cmp::max(v, score)),
                None => Some(score),
            };
        } else if let Some(v) = get_best(&bank[i + 1..], digits - 1, memo) {
            let score = bank[i] * 10i64.pow((digits - 1) as u32) + v;
            max = match max {
                Some(v) => Some(std::cmp::max(v, score)),
                None => Some(score),
            };
        }
    }
    memo.insert((bank.len(), digits), max.unwrap());
    max
}
