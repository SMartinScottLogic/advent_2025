use nalgebra::{DMatrix, SMatrix, matrix, vector};
use z3::{Config, Context, SatResult, ast};
use std::{
    collections::VecDeque,
    io::{BufRead, BufReader},
};
#[allow(unused_imports)]
use tracing::{debug, event_enabled, info, Level};

pub type ResultType = u64;

#[derive(Debug, Default)]
pub(crate) struct Machine {
    lights: String,
    buttons: Vec<Vec<usize>>,
    joltage: String,
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
            info!("{} for {:?}", presses, machine);
            total += presses;
        }
        Ok(total as ResultType)
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
        let mut total = 0;
        for machine in self.machines.iter().take(10000) {
            let presses = solve_p2(machine);
            info!("{} for {:?}", presses, machine);
            //let presses = fewest_presses_joltage(machine);
            //info!("{} for {:?}", presses, machine);

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
    let buttons = machine.buttons;
    let jolts = machine.joltage.split(',')
        .map(|v| v.parse::<u64>().unwrap()).collect::<Vec<_>>();

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
    for (e, &j) in exprs.into_iter().zip(jolts) {
        problem.add_constraint(e.eq(j as f64));
    }
    let sol = problem.solve().unwrap();
    press_vars.iter().map(|&v| sol.value(v)).sum::<f64>() as _
}

// fn fewest_presses_joltage(machine: &Machine) -> u64 {
//     // https://docs.rs/z3/latest/z3/
//     // https://github.com/jonathanpaulson/AdventOfCode/blob/master/2025/10.py
// //Z3
// let ctx = Context::new(&Config::new());
// let solver = Solver::new(ctx);

// // Buttons
// let buttons = machine.buttons
//     .iter()
//     .enumerate()
//     .map(|(id, _)| ast::Int::new_const(&ctx, format!("presses_{}",id+1)))
//     .collect::<Vec<_>>();
// for (i, joltage) in machine
//         .joltage
//         .split(',')
//         .map(|v| v.parse::<u64>().unwrap())
//         .enumerate() {
//             let terms = 0;
//            for (button_id, button_effect) in machine.buttons.iter().enumerate() {
//             if button_effect.contains(i) {
//                 terms = terms + buttons.get(button_id).unwrap();
//             }
//            } 
//            solver.assert(terms.eq(joltage));
// }
// match solution.check() {
//     SatResult::Sat => {
//         let model = solver.get_model().unwrap();
//         debug!(model = ?model);
//     }
//     _ => panic!()
// }
// 0
//     // // nalgebra
//     // let m =
//     //     matrix![button_a.x() as f64, button_b.x() as f64; button_a.y() as f64, button_b.y() as f64];
//     // match m.try_inverse() {
//     //     Some(inv) => {
//     //         let r = inv * vector![prize.x() as f64, prize.y() as f64];
//     //         debug!(?r);
//     //         if r.iter().all(|f| (f - f.round()).abs() < 1e-3) {
//     //             let r = r.transpose() * vector![3.0, 1.0];
//     //             debug!(?r);
//     //             Some(r.magnitude().round() as u64)
//     //         } else {
//     //             None
//     //         }
//     //     }
//     //     None => None,
//     // }

//     // let target = machine
//     //     .joltage
//     //     .split(',')
//     //     .map(|v| v.parse::<u64>().unwrap())
//     //     .collect::<Vec<_>>();
//     // let joltage = "0".repeat(machine.lights.len());
//     // let joltage = joltage.chars().map(|_| 0).collect::<Vec<_>>();
//     // fewest_presses_joltage_r(machine, &target, 0, 0, &joltage).unwrap()
//     // let mut queue = VecDeque::new();
//     // queue.push_back((0_u64, 0, joltage));

//     // let mut min_presses = u64::MAX;
//     // while let Some((presses, button, joltage)) = queue.pop_front() {
//     //     if joltage == target {
//     //         debug!("found for {}", presses);
//     //         if presses < min_presses {
//     //             min_presses = presses;
//     //         }
//     //         continue;
//     //     }

//     //     info!(
//     //         "min_presses: {}, presses: {}, button: {}, joltage: {:?}",
//     //         min_presses, presses, button, joltage
//     //     );
//     //     if let Some(lights) = machine.buttons.get(button) {
//     //         let mut max_presses = u64::MAX;
//     //         for light in lights {
//     //             let diff = target.get(*light).unwrap() - joltage.get(*light).unwrap();
//     //             max_presses = std::cmp::min(max_presses, diff);
//     //         }
//     //         debug!(max_presses);
//     //         for press in (0..=max_presses).rev() {
//     //             let mut b_joltage = joltage.clone();

//     //             for light in lights {
//     //                 b_joltage[*light] += press;
//     //             }
//     //             if b_joltage
//     //                 .iter()
//     //                 .zip(target.iter())
//     //                 .any(|(actual, target)| actual > target)
//     //             {
//     //                 panic!();
//     //             }
//     //             queue.push_back((presses + press, button + 1, b_joltage));
//     //         }
//     //     }
//     // }
//     // min_presses
// }
