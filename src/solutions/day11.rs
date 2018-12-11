use solver::Solver;
use std::cmp::min;
use std::collections::HashMap;
use std::io;

pub struct Day11;

impl Solver for Day11 {
    type Input = i32;
    type Output1 = String;
    type Output2 = String;

    fn day() -> u32 {
        11
    }

    fn parse_input<R: io::Read>(mut r: R) -> i32 {
        let mut s = String::new();
        r.read_to_string(&mut s).expect("unable to read to string");
        s.parse().unwrap()
    }

    fn solve_first(input: &i32) -> String {
        let powers = compute_all_powers(*input);
        let mut max_square: Option<SquarePower> = None;

        for y in 1..=298 {
            for x in 1..=298 {
                let mut sum = 0;
                for j in 0..3 {
                    for i in 0..3 {
                        let pt = Pt { x: x + i, y: y + j };
                        let power = powers.get(&pt).unwrap();
                        sum += power;
                    }
                }
                let current_square = SquarePower {
                    pt: Pt { x, y },
                    size: 3,
                    power: sum,
                };
                let max_pow = max_square
                    .as_ref()
                    .and_then(|sp| Some(sp.power))
                    .unwrap_or(0);
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

    fn solve_second(input: &i32) -> String {
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
                        let pt = Pt { x: x + i, y: y + (n - 1) };
                        let power = powers.get(&pt).unwrap();
                        current_sum += power;
                    }
                    for i in 0..n-1 {
                        let pt = Pt { x: x + (n - 1), y: y + i };
                        let power = powers.get(&pt).unwrap();
                        current_sum += power;
                    }

                    let current_square = SquarePower {
                        pt: Pt { x, y },
                        size: n as usize,
                        power: current_sum,
                    };

                    /*println!(
                        "{},{},{},{}",
                        current_square.pt.x,
                        current_square.pt.y,
                        current_square.size,
                        current_square.power,
                    );*/

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

fn compute_all_powers(serial_number: i32) -> HashMap<Pt, i32> {
    let mut map = HashMap::new();
    for y in 1..=300 {
        for x in 1..=300 {
            let pt = Pt { x, y };
            let pow = power(&pt, serial_number);
            map.insert(pt, pow);
        }
    }
    map
}

struct SquarePower {
    pt: Pt,
    size: usize,
    power: i32,
}
