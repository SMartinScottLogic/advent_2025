use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    io::{BufRead, BufReader},
};
#[allow(unused_imports)]
use tracing::{debug, event_enabled, info, Level};

pub type ResultType = u64;

#[derive(Default)]
pub struct Solution {
    junctionboxes: Vec<(i64, i64, i64)>,
    distances: Vec<(usize, usize, i64)>,
}
impl Debug for Solution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Solution")
            .field("junctionboxes", &self.junctionboxes.len())
            .field("distances", &self.distances.len())
            .finish()
    }
}
impl Solution {
    pub fn add_junctionbox(&mut self, x: i64, y: i64, z: i64) {
        self.junctionboxes.push((x, y, z));
    }
}

#[allow(unused_variables, unused_mut)]
impl<T: std::io::Read> TryFrom<BufReader<T>> for Solution {
    type Error = std::io::Error;

    fn try_from(reader: BufReader<T>) -> Result<Self, Self::Error> {
        let mut solution = Self::default();
        for (id, line) in reader.lines().map_while(Result::ok).enumerate() {
            // Implement for problem
            let (x, y, z) = line
                .split(',')
                .map(|v| v.parse::<i64>().unwrap())
                .collect_tuple()
                .unwrap();
            solution.add_junctionbox(x, y, z);
        }
        Ok(solution)
    }
}
impl utils::Solution for Solution {
    type Result = anyhow::Result<ResultType>;
    fn analyse(&mut self, _is_full: bool) {
        self.distances = Vec::new();
        for (i, (x1, y1, z1)) in self.junctionboxes.iter().enumerate() {
            for (j, (x2, y2, z2)) in self.junctionboxes.iter().enumerate().take(i) {
                if j >= i {
                    panic!();
                }
                let distance = (x1 - x2).pow(2) + (y1 - y2).pow(2) + (z1 - z2).pow(2);
                self.distances.push((i, j, distance));
            }
        }
        self.distances.sort_by_key(|(_i, _j, distance)| -distance);
    }

    fn answer_part1(&self, is_full: bool) -> Self::Result {
        let mut distances = self.distances.clone();

        let limit = if is_full { 1000 } else { 10 };
        let mut connections = HashSet::new();

        for _ in 0..limit {
            let (i, j, _distance) = distances.pop().unwrap();
            connections.insert((i, j));
        }
        debug!(?connections);

        let mut groups: HashMap<usize, HashSet<usize>> = HashMap::new();
        let mut group_id = HashMap::new();
        for (i, j) in connections {
            debug!(i, j);
            if let Some(&groupid_i) = group_id.get(&i) {
                if let Some(&groupid_j) = group_id.get(&j) {
                    if groupid_i != groupid_j {
                        let target_id = std::cmp::min(groupid_i, groupid_j);
                        let source_id = std::cmp::max(groupid_i, groupid_j);
                        let s = groups.insert(source_id, HashSet::new()).unwrap();
                        let e = groups.entry(target_id).or_default();
                        for v in s {
                            e.insert(v);
                            group_id.insert(v, target_id);
                        }
                    }
                } else {
                    groups.entry(groupid_i).or_default().insert(j);
                    group_id.insert(j, groupid_i);
                }
            } else if let Some(&groupid_j) = group_id.get(&j) {
                groups.entry(groupid_j).or_default().insert(i);
                group_id.insert(i, groupid_j);
            } else {
                let gid = group_id.len();
                groups.entry(gid).or_default().insert(i);
                groups.entry(gid).or_default().insert(j);
                group_id.insert(i, gid);
                group_id.insert(j, gid);
            }
        }
        let mut groups_len = groups.values().map(|group| group.len()).collect::<Vec<_>>();
        groups_len.sort_by_key(|v| -(*v as isize));
        let r = groups_len.iter().take(3).product::<usize>();
        // Implement for problem
        Ok(r as ResultType)
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
        // Put each box in its own group
        let mut group_id =
            self.junctionboxes
                .iter()
                .enumerate()
                .fold(HashMap::new(), |mut acc, (id, _pos)| {
                    acc.insert(id, id);
                    acc
                });
        let mut groups: HashMap<usize, HashSet<usize>> = self
            .junctionboxes
            .iter()
            .enumerate()
            .fold(HashMap::new(), |mut acc, (id, _pos)| {
                acc.entry(id).or_default().insert(id);
                acc
            });

        let mut distances = self.distances.clone();

        let mut last_ids;
        loop {
            // join closest unconnected pair
            let (i, j, _distance) = distances.pop().unwrap();
            let groupid_i = group_id.get(&i).unwrap();
            let groupid_j = group_id.get(&j).unwrap();
            if groupid_i == groupid_j {
                continue;
            }
            last_ids = Some((i, j));
            // Merge groups
            let target_id = std::cmp::min(*groupid_i, *groupid_j);
            let source_id = std::cmp::max(*groupid_i, *groupid_j);
            let s = groups.insert(source_id, HashSet::new()).unwrap();
            let e = groups.entry(target_id).or_default();
            for v in s {
                e.insert(v);
                group_id.insert(v, target_id);
            }
            if groups
                .iter()
                .filter(|(_id, group)| !group.is_empty())
                .count()
                == 1
            {
                break;
            }
        }

        let r = [last_ids.unwrap().0, last_ids.unwrap().1]
            .iter()
            .map(|&id| self.junctionboxes.get(id).unwrap().0)
            .product::<i64>();
        // Implement for problem
        Ok(r as ResultType)
    }
}
