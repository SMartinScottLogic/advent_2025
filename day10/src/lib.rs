use std::{
    collections::{HashMap, VecDeque},
    io::{BufRead, BufReader},
};
#[allow(unused_imports)]
use tracing::{debug, event_enabled, info, Level};

pub type ResultType = u64;

#[derive(Debug, Default)]
pub(crate) struct Machine {
    lights: String,
    buttons: Vec<Vec<usize>>,
    joltage: Vec<usize>,
}
#[derive(Debug, Default)]
pub struct Solution {
    machines: Vec<Machine>,
}
impl Solution {
    pub(crate) fn add_machine(&mut self, machine: Machine) {
        self.machines.push(machine);
    }
}

#[allow(unused_variables, unused_mut)]
impl<T: std::io::Read> TryFrom<BufReader<T>> for Solution {
    type Error = std::io::Error;

    fn try_from(reader: BufReader<T>) -> Result<Self, Self::Error> {
        let mut solution = Self::default();
        for (id, line) in reader.lines().map_while(Result::ok).enumerate() {
            let line = line.trim();
            let mut machine = Machine::default();
            for part in line.split(' ') {
                match part.chars().next().unwrap() {
                    '[' => {
                        machine.lights = part
                            .chars()
                            .skip(1)
                            .take(part.len() - 2)
                            .collect::<String>()
                    }
                    '(' => {
                        let mut buttons = part
                            .chars()
                            .skip(1)
                            .take(part.len() - 2)
                            .collect::<String>()
                            .split(',')
                            .map(|v| v.parse::<usize>().unwrap())
                            .collect::<Vec<_>>();
                        machine.buttons.push(buttons);
                    }
                    '{' => {
                        machine.joltage = part
                            .chars()
                            .skip(1)
                            .take(part.len() - 2)
                            .collect::<String>()
                            .split(',')
                            .map(|v| v.parse::<usize>().unwrap())
                            .collect::<Vec<_>>();
                    }
                    c => panic!("unexpected {}", c),
                }
            }
            // Implement for problem
            solution.add_machine(machine);
        }
        Ok(solution)
    }
}

impl utils::Solution for Solution {
    type Result = anyhow::Result<ResultType>;
    fn analyse(&mut self, _is_full: bool) {}

    fn answer_part1(&self, _is_full: bool) -> Self::Result {
        let mut total = 0;
        for machine in &self.machines {
            let presses = fewest_presses_lights(machine);
            debug!("{} for {:?}", presses, machine);
            total += presses;
        }
        Ok(total as ResultType)
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
        let mut total = 0;
        for machine in self.machines.iter().take(10000) {
            let presses = solve_p2(machine);
            debug!("{} for {:?}", presses, machine);

            total += presses;
        }
        Ok(total as ResultType)
    }
}

fn fewest_presses_lights(machine: &Machine) -> usize {
    let target = machine.lights.bytes().collect::<Vec<_>>();
    let mut queue = VecDeque::new();
    let lights = ".".repeat(machine.lights.len());
    let lights = lights.bytes().collect::<Vec<_>>();
    queue.push_back((0, lights));

    while let Some((presses, lights)) = queue.pop_front() {
        if lights == target {
            return presses;
        }
        debug!("presses: {}, lights: {:?}", presses, lights);
        for buttons in &machine.buttons {
            let mut b_lights = lights.clone();
            for button in buttons {
                let l = match b_lights[*button] {
                    b'.' => b'#',
                    b'#' => b'.',
                    c => panic!("unexpected {}", c),
                };
                b_lights[*button] = l;
            }
            queue.push_back((presses + 1, b_lights));
        }
    }
    0
}

fn solve_p2(machine: &Machine) -> usize {
    let buttons = &machine.buttons;
    let jolts = machine.joltage.clone();

    use good_lp::*;
    let mut vars = variables!();
    let press_vars = (0..buttons.len())
        .map(|_| vars.add(variable().min(0).integer()))
        .collect::<Vec<_>>();

    let mut problem = highs(vars.minimise(press_vars.iter().sum::<Expression>()));
    let mut exprs = vec![0.into_expression(); jolts.len()];
    for i in 0..buttons.len() {
        for &x in &buttons[i] {
            exprs[x] += press_vars[i];
        }
    }
    for (e, j) in exprs.into_iter().zip(jolts) {
        problem.add_constraint(e.eq(j as f64));
    }
    let sol = problem.solve().unwrap();

    // Validate
    let mut outcome = HashMap::new();
    let mut total = 0;
    for (i, p) in press_vars.iter().map(|&v| sol.value(v)).enumerate() {
        let op = p;
        let p = (p + 0.9) as isize;
        debug!("{}: {} vs {}", i, p, op);
        assert!(p >= 0);
        for j in buttons.get(i).unwrap() {
            *outcome.entry(j).or_insert(0) += p as usize;
        }
        total += p;
    }
    let mut result = Vec::new();
    for i in 0..machine.joltage.len() {
        result.push(*outcome.get(&i).unwrap());
    }
    assert_eq!(result, machine.joltage);

    total as usize
}
