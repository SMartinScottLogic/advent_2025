use std::{
    cmp::{max, min},
    collections::HashMap,
    io::{BufRead, BufReader},
};
#[allow(unused_imports)]
use tracing::{debug, event_enabled, info, Level};
use utils::{grid::Matrix, point::Point};

pub type ResultType = u64;

#[derive(Debug, Clone, Copy)]
struct CompactTile {
    original: Point<isize>,
    compact: Point<isize>,
}
#[derive(Debug, Default)]
pub struct Solution {
    tiles: Vec<Point<isize>>,
    compact_tiles: Vec<CompactTile>,
}
impl Solution {
    pub fn add_tile(&mut self, x: isize, y: isize) {
        self.tiles.push(Point::new(x, y))
    }
}

#[allow(unused_variables, unused_mut)]
impl<T: std::io::Read> TryFrom<BufReader<T>> for Solution {
    type Error = std::io::Error;

    fn try_from(reader: BufReader<T>) -> Result<Self, Self::Error> {
        let mut solution = Self::default();
        for (id, line) in reader.lines().map_while(Result::ok).enumerate() {
            // Implement for problem
            let (x, y) = line.split_once(',').unwrap();
            let x = x.parse().unwrap();
            let y = y.parse().unwrap();
            solution.add_tile(x, y);
        }
        Ok(solution)
    }
}
impl utils::Solution for Solution {
    type Result = anyhow::Result<ResultType>;
    fn analyse(&mut self, _is_full: bool) {
        let mut all_x = Vec::new();
        let mut all_y = Vec::new();
        for p in &self.tiles {
            all_x.push(p.x());
            all_y.push(p.y());
        }
        all_x.sort();
        all_y.sort();
        all_x.dedup();
        all_y.dedup();

        let x_indices = all_x
            .iter()
            .enumerate()
            .fold(HashMap::new(), |mut acc, (idx, &v)| {
                acc.insert(v, 2 * idx as isize);
                acc
            });
        let y_indices = all_y
            .iter()
            .enumerate()
            .fold(HashMap::new(), |mut acc, (idx, &v)| {
                acc.insert(v, 2 * idx as isize);
                acc
            });
        info!("x_indices = {:?}", x_indices);
        info!("y_indices = {:?}", y_indices);
        for p in &self.tiles {
            self.compact_tiles.push(CompactTile {
                original: *p,
                compact: Point::new(
                    *x_indices.get(&p.x()).unwrap(),
                    *y_indices.get(&p.y()).unwrap(),
                ),
            });
        }
    }

    fn answer_part1(&self, _is_full: bool) -> Self::Result {
        let mut area = 0;
        for p1 in self.tiles.iter() {
            for p2 in self.tiles.iter() {
                area = max(area, (1 + p1.x() - p2.x()) * (1 + p1.y() - p2.y()));
            }
        }
        // Implement for problem
        Ok(area as ResultType)
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
        let mut areas = Vec::new();
        for tile1 in self.compact_tiles.iter() {
            for tile2 in self.compact_tiles.iter() {
                let area = (1 + (tile1.original.x() - tile2.original.x()).abs())
                    * (1 + (tile1.original.y() - tile2.original.y()).abs());
                areas.push((area, *tile1, *tile2));
            }
        }
        areas.sort_by_key(|(area, ..)| -area);
        for area in &areas {
            debug!("area: {} {:?} {:?}", area.0, area.1, area.2);
        }

        // Border
        let mut matrix = Matrix::new();
        for i in 0..self.tiles.len() {
            let tile1 = self.compact_tiles.get(i).unwrap();
            let next = (i + 1) % self.compact_tiles.len();
            let tile2 = self.compact_tiles.get(next).unwrap();
            if tile1.compact.x() == tile2.compact.x() {
                for gy in min(tile1.compact.y(), tile2.compact.y())
                    ..=max(tile1.compact.y(), tile2.compact.y())
                {
                    matrix.set(tile1.compact.x(), gy, 'X');
                }
            } else if tile1.compact.y() == tile2.compact.y() {
                for gx in min(tile1.compact.x(), tile2.compact.x())
                    ..=max(tile1.compact.x(), tile2.compact.x())
                {
                    matrix.set(gx, tile1.compact.y(), 'X');
                }
            } else {
                panic!();
            }
            matrix.set(tile1.compact.x(), tile1.compact.y(), '#');
            matrix.set(tile2.compact.x(), tile2.compact.y(), '#');
        }

        if tracing::enabled!(Level::DEBUG) {
            matrix.display_with_mapping(|v| {
                if v == '#' || v == 'X' || v == 'O' {
                    v.to_string()
                } else {
                    ' '.to_string()
                }
            });
        }

        // Flood outside
        let (max_x, max_y) = matrix.dimensions();
        let start = Point::new(-1, -1);
        let mut queue = Vec::new();
        queue.push(start);
        matrix.set(start.x(), start.y(), 'O');
        while let Some(p) = queue.pop() {
            for n in p.cardinal() {
                if n.x() >= -1 && n.y() >= -1 && n.x() <= max_x + 1 && n.y() <= max_y + 1 {
                    debug!("probe: {:?}", n);
                    if matrix.get(n.x(), n.y()).is_none() {
                        matrix.set(n.x(), n.y(), 'O');
                        queue.push(n);
                    }
                }
            }
        }

        if tracing::enabled!(Level::DEBUG) {
            matrix.display_with_mapping(|v| {
                if v == '#' || v == 'X' || v == 'O' {
                    v.to_string()
                } else {
                    ' '.to_string()
                }
            });
        }
        let area = best_area(&areas, &matrix);

        // low: 39925070
        // low: 107849020
        // not: 1098764625
        // [p]: 1574684850
        // p1 : 4750176210
        // Implement for problem
        Ok(area as ResultType)
    }
}

fn is_permitted(matrix: &Matrix<char>, tile1: &CompactTile, tile2: &CompactTile) -> bool {
    for x in min(tile1.compact.x(), tile2.compact.x())..=max(tile1.compact.x(), tile2.compact.x()) {
        for y in
            min(tile1.compact.y(), tile2.compact.y())..=max(tile1.compact.y(), tile2.compact.y())
        {
            if let Some('O') = matrix.get(x, y) {
                return false;
            }
        }
    }
    true
}

fn best_area(areas: &[(isize, CompactTile, CompactTile)], matrix: &Matrix<char>) -> isize {
    for (area, tile1, tile2) in areas.iter() {
        let valid = is_permitted(matrix, tile1, tile2);
        if valid {
            return *area;
        } else {
            debug!("reject: {} {:?} {:?}", area, tile1, tile2);
        }
    }
    panic!();
}
