use std::io::{BufRead, BufReader};
#[allow(unused_imports)]
use tracing::{debug, event_enabled, info, Level};

pub type ResultType = u64;

#[derive(Debug, Default)]
pub struct Solution {
    freshlist: Vec<(u64, u64)>,
    ingredients: Vec<u64>,
}
impl Solution {
    pub fn add_ingredient(&mut self, ingredient: u64) {
        self.ingredients.push(ingredient);
    }
    pub fn add_freshlist(&mut self, start: u64, end: u64) {
        self.freshlist.push((start, end));
    }
}

#[allow(unused_variables, unused_mut)]
impl<T: std::io::Read> TryFrom<BufReader<T>> for Solution {
    type Error = std::io::Error;

    fn try_from(reader: BufReader<T>) -> Result<Self, Self::Error> {
        let mut solution = Self::default();
        for (id, line) in reader.lines().map_while(Result::ok).enumerate() {
            // Implement for problem
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            match line.split_once('-') {
                Some((a, b)) => {
                    let start = a.parse().unwrap();
                    let end = b.parse().unwrap();
                    solution.add_freshlist(start, end);
                }
                None => {
                    let ingredient = line.parse().unwrap();
                    solution.add_ingredient(ingredient);
                }
            }
        }
        Ok(solution)
    }
}
impl utils::Solution for Solution {
    type Result = anyhow::Result<ResultType>;
    fn analyse(&mut self, _is_full: bool) {}

    fn answer_part1(&self, _is_full: bool) -> Self::Result {
        let mut count = 0;
        for ingredient in &self.ingredients {
            if is_fresh(*ingredient, &self.freshlist) {
                count += 1;
            }
        }
        // Implement for problem
        Ok(count)
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
        let mut ranges = Vec::new();
        for (start, end) in &self.freshlist {
            ranges = add_range(&ranges, *start, *end);
        }
        let mut count = 0;
        for (start, end) in ranges.iter() {
            count += 1 + end - start;
        }
        debug!(?ranges, count);
        // Implement for problem
        Ok(count as ResultType)
    }
}

fn is_fresh(ingredient: u64, freshlist: &[(u64, u64)]) -> bool {
    freshlist
        .iter()
        .any(|(start, end)| (*start..=*end).contains(&ingredient))
}

fn add_range(ranges: &[(u64, u64)], start: u64, end: u64) -> Vec<(u64, u64)> {
    let mut new_ranges = Vec::new();
    let mut start = start;
    let mut end = end;
    for (rs, re) in ranges.iter() {
        if start < *rs && end < *rs {
            new_ranges.push((*rs, *re));
            continue;
        }
        if start > *re && end > *re {
            new_ranges.push((*rs, *re));
            continue;
        }
        // Must overlap
        debug!("overlap ({} {}) ({} {})", *rs, *re, start, end);
        start = std::cmp::min(start, *rs);
        end = std::cmp::max(end, *re);
    }
    debug!(start, end, "new");
    new_ranges.push((start, end));
    new_ranges
}
