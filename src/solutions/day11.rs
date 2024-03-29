use crate::solver::Solver;
use std::{cmp::min, io};

pub struct Problem;

impl Solver for Problem {
    type Input = i32;
    type Output1 = String;
    type Output2 = String;

    fn parse_input<R: io::Read>(&self, mut r: R) -> i32 {
        let mut s = String::new();
        r.read_to_string(&mut s).expect("unable to read to string");
        s.parse().unwrap()
    }

    fn solve_first(&self, input: &i32) -> String {
        let powers = compute_all_powers(*input);
        let mut max_square: Option<SquarePower> = None;

        for y in 1..=298 {
            for x in 1..=298 {
                let mut sum = 0;
                for j in 0..3 {
                    for i in 0..3 {
                        let pt = Pt { x: x + i, y: y + j };
                        let power = get_power(&powers, &pt);
                        sum += power;
                    }
                }
                let current_square = SquarePower {
                    pt: Pt { x, y },
                    size: 3,
                    power: sum,
                };
                let max_pow = max_square.as_ref().map(|sp| sp.power).unwrap_or(0);
                if max_pow < sum {
                    max_square = Some(current_square);
                }
            }
        }

        format!(
            "{},{}",
            max_square.as_ref().unwrap().pt.x,
            max_square.as_ref().unwrap().pt.y
        )
    }

    fn solve_second(&self, input: &i32) -> String {
        let powers = compute_all_powers(*input);
        let mut max_square: Option<SquarePower> = None;

        for y in 1..=300 {
            for x in 1..=300 {
                let max_width = 301 - x;
                let max_height = 301 - y;
                let size = min(max_width, max_height);

                let mut current_sum = 0;
                for n in 1..=size {
                    for i in 0..n {
                        let pt = Pt {
                            x: x + i,
                            y: y + (n - 1),
                        };
                        let power = get_power(&powers, &pt);
                        current_sum += power;
                    }
                    for i in 0..n - 1 {
                        let pt = Pt {
                            x: x + (n - 1),
                            y: y + i,
                        };
                        let power = get_power(&powers, &pt);
                        current_sum += power;
                    }

                    let current_square = SquarePower {
                        pt: Pt { x, y },
                        size: n as usize,
                        power: current_sum,
                    };

                    let max_pow = max_square.as_ref().map(|sp| sp.power).unwrap_or(0);
                    if max_pow < current_sum {
                        max_square = Some(current_square);
                    }
                }
            }
        }

        format!(
            "{},{},{} --> {}",
            max_square.as_ref().unwrap().pt.x,
            max_square.as_ref().unwrap().pt.y,
            max_square.as_ref().unwrap().size,
            max_square.as_ref().unwrap().power,
        )
    }
}

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct Pt {
    x: i32,
    y: i32,
}

fn power(pt: &Pt, serial_number: i32) -> i32 {
    let rack_id = pt.x + 10;
    let p = (rack_id * pt.y + serial_number) * rack_id;
    (p / 100) % 10 - 5
}

fn compute_all_powers(serial_number: i32) -> Vec<i32> {
    let mut sol = Vec::with_capacity(300 * 300);
    for y in 1..=300 {
        for x in 1..=300 {
            let pt = Pt { x, y };
            let pow = power(&pt, serial_number);
            sol.push(pow);
        }
    }
    sol
}

#[inline]
fn get_power(powers: &[i32], pt: &Pt) -> i32 {
    let i = (pt.x - 1) + (pt.y - 1) * 300;
    *powers.get(i as usize).unwrap_or(&0)
}

struct SquarePower {
    pt: Pt,
    size: usize,
    power: i32,
}
