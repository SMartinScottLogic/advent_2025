use std::fmt::Debug;
use std::io::{BufRead, BufReader};
use tracing::enabled;
#[allow(unused_imports)]
use tracing::{debug, event_enabled, info, Level};
use utils::grid::Matrix;

pub type ResultType = u64;

pub(crate) struct Region {
    width: isize,
    height: isize,
    present_counts: Vec<u64>,
}
impl Debug for Region {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Region").finish()
    }
}

#[derive(Clone)]
pub(crate) struct Present {
    shape: Matrix<char>,
}
impl Debug for Present {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Present").finish()
    }
}
impl Present {
    fn rotate(&self) -> Self {
        let mut rotated_shape = Matrix::new();
        let (maxx, maxy) = self.shape.dimensions();
        for y in 0..=maxy {
            for x in 0..=maxx {
                let value = self.shape.get(x, y).unwrap();
                rotated_shape.set(maxy - y, x, *value);
            }
        }
        Self {
            shape: rotated_shape,
        }
    }

    fn flip(&self) -> Self {
        let mut flipped_shape = Matrix::new();
        let (maxx, maxy) = self.shape.dimensions();
        for y in 0..=maxy {
            for x in 0..=maxx {
                let value = self.shape.get(x, y).unwrap();
                flipped_shape.set(x, maxy - y, *value);
            }
        }
        Self {
            shape: flipped_shape,
        }
    }
}

#[derive(Default)]
pub struct Solution {
    presents: Vec<Present>,
    regions: Vec<Region>,
}
impl Debug for Solution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Solution").finish()
    }
}

impl Solution {
    pub(crate) fn add_present(&mut self, v: &[String]) {
        let mut shape = Matrix::new();
        for (y, row) in v.iter().enumerate() {
            for (x, c) in row.chars().enumerate() {
                shape.set(x as isize, y as isize, c);
            }
        }
        self.presents.push(Present { shape });
    }
    pub(crate) fn add_region(&mut self, width: isize, height: isize, shapes: Vec<u64>) {
        self.regions.push(Region {
            width,
            height,
            present_counts: shapes,
        });
    }
}

#[allow(unused_variables, unused_mut)]
impl<T: std::io::Read> TryFrom<BufReader<T>> for Solution {
    type Error = std::io::Error;

    fn try_from(reader: BufReader<T>) -> Result<Self, Self::Error> {
        let mut solution = Self::default();
        let mut shape = Vec::new();
        for (id, line) in reader.lines().map_while(Result::ok).enumerate() {
            // Implement for problem
            let line = line.trim();
            if line.is_empty() {
                if !shape.is_empty() {
                    solution.add_present(&shape);
                    shape.clear();
                }
                continue;
            }
            match line.split_once(':') {
                None => shape.push(line.to_string()),
                Some((lhs, rhs)) => {
                    let lhs = lhs.trim();
                    let rhs = rhs.trim();
                    if rhs.is_empty() {
                        // Id - ignore
                    } else {
                        let (width, height) = lhs.split_once('x').unwrap();
                        let width = width.parse().unwrap();
                        let height = height.parse().unwrap();
                        let quantity = rhs
                            .split(' ')
                            .map(|s| s.parse::<u64>().unwrap())
                            .collect::<Vec<_>>();
                        solution.add_region(width, height, quantity);
                    }
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
        let mut r = 0;
        let mut maybe_r = 0;
        for region in &self.regions {
            if may_fit_presents(region, &self.presents) {
                maybe_r += 1;
            }
        }
        for region in &self.regions {
            if may_fit_presents(region, &self.presents) {
                debug!("checking region {:?}", region);
                if can_fit_presents(region, &self.presents) {
                    debug!("presents fit in region {:?}", region);
                    r += 1;
                }
            }
        }
        // Implement for problem
        info!(r, maybe_r);
        Ok(r)
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
        // Implement for problem
        Ok(0)
    }
}

fn may_fit_presents(region: &Region, presents: &[Present]) -> bool {
    let area = (region.width * region.height) as i64;
    let mut total_present_area = 0;
    for (id, count) in region.present_counts.iter().enumerate() {
        let present = &presents[id].shape;
        let (p_maxx, p_maxy) = present.dimensions();
        for y in 0..=p_maxy {
            for x in 0..=p_maxx {
                if let Some('#') = present.get(x, y) {
                    total_present_area += *count as i64;
                }
            }
        }
    }
    debug!(
        area,
        total_present_area,
        diff = area - total_present_area,
        rat = (area as f64) / (total_present_area as f64),
        "probe"
    );
    (area - total_present_area) > 0
}
fn can_fit_presents(region: &Region, presents: &[Present]) -> bool {
    let mut space = Matrix::new();
    for y in 0..region.height {
        for x in 0..region.width {
            space.set(x, y, '.');
        }
    }
    presents_fit(space, &region.present_counts, presents)
}

fn presents_fit(space: Matrix<char>, remaining_counts: &[u64], presents: &[Present]) -> bool {
    let r = remaining_counts.iter().enumerate().find(|(_i, v)| **v > 0);
    match r {
        None => {
            if enabled!(Level::DEBUG) {
                space.display();
            }
            true
        }
        Some((idx, _count)) => {
            // Find all possible rotations / flips / positions :(
            let present = presents.get(idx).unwrap();
            let mut present = present.to_owned();
            for rotate in 1..=4 {
                present = present.rotate();
                if enabled!(Level::DEBUG) {
                    info!("rotate {}", rotate);
                    present.shape.display();
                }
                // No flip
                for (x, y) in get_all_permitted(&space, &present) {
                    // Mark populated cells
                    let mut new_space = space.clone();
                    if enabled!(Level::DEBUG) {
                        info!("===== before =====");
                        new_space.display();
                        info!("{:?} @ ({},{})", present, x, y);
                        present.shape.display();
                        info!("---");
                    }
                    mark_populated(&mut new_space, &present, x, y);
                    if enabled!(Level::DEBUG) {
                        info!("===== after =====");
                        new_space.display();
                    }
                    // Decrease remaining count
                    let mut new_remaining_counts = remaining_counts.to_vec();
                    new_remaining_counts[idx] -= 1;
                    // Does remainder fit?
                    if presents_fit(new_space, &new_remaining_counts, presents) {
                        return true;
                    }
                }
                // Flip
                present = present.flip();

                // Flip back for next loop
                present = present.flip();
            }
            false
        }
    }
}

fn get_all_permitted(space: &Matrix<char>, present: &Present) -> Vec<(isize, isize)> {
    if event_enabled!(Level::DEBUG) {
        info!("all permitted");
        info!("space");
        space.display();
        info!("present");
        present.shape.display();
    }
    let mut all_permitted = Vec::new();
    let (maxx, maxy) = space.dimensions();
    for y in 0..=maxy {
        for x in 0..=maxx {
            if !present_overlaps(space, present, x, y) {
                all_permitted.push((x, y));
            }
        }
    }
    debug!("all_permitted: {:?}", all_permitted);
    all_permitted
}

fn present_overlaps(space: &Matrix<char>, present: &Present, x: isize, y: isize) -> bool {
    let (present_maxx, present_maxy) = present.shape.dimensions();
    for py in 0..=present_maxy {
        for px in 0..=present_maxx {
            if let Some('#') = present.shape.get(px, py) {
                if matches!(space.get(x + px, y + py), None | Some('#')) {
                    return true;
                }
            }
        }
    }
    false
}

fn mark_populated(space: &mut Matrix<char>, present: &Present, x: isize, y: isize) {
    let (present_maxx, present_maxy) = present.shape.dimensions();
    for py in 0..=present_maxy {
        for px in 0..=present_maxx {
            if let Some('#') = present.shape.get(px, py) {
                let mx = x + px;
                let my = y + py;
                if let Some('.') = space.get(mx, my) {
                    space.set(mx, my, '#');
                } else {
                    panic!();
                }
            }
        }
    }
}
