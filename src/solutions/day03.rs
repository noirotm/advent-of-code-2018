use crate::solver::Solver;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io;
use std::io::BufRead;
use std::io::BufReader;

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<Rectangle>;
    type Output1 = u64;
    type Output2 = u32;

    fn parse_input<R: io::Read>(&self, r: R) -> Vec<Rectangle> {
        let re = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").expect("Invalid regex");

        BufReader::new(r)
            .lines()
            .filter_map(|l| l.ok())
            .filter_map(|s| {
                re.captures(s.as_str()).and_then(|c| {
                    Some(Rectangle {
                        id: c.get(1)?.as_str().parse().ok()?,
                        left: c.get(2)?.as_str().parse().ok()?,
                        top: c.get(3)?.as_str().parse().ok()?,
                        width: c.get(4)?.as_str().parse().ok()?,
                        height: c.get(5)?.as_str().parse().ok()?,
                    })
                })
            })
            .collect()
    }

    fn solve_first(&self, input: &Vec<Rectangle>) -> u64 {
        let mut occupied_areas = HashMap::new();
        let mut sol_area = 0;

        for r in input {
            for x in r.left..r.left + r.width {
                for y in r.top..r.top + r.height {
                    let k = (x, y);
                    let v: u64 = occupied_areas.get(&k).unwrap_or(&0) + 1;
                    occupied_areas.insert(k, v);
                    if v == 2 {
                        sol_area += 1;
                    }
                }
            }
        }

        sol_area
    }

    fn solve_second(&self, input: &Vec<Rectangle>) -> u32 {
        let mut overlaps = HashSet::new();

        for r1 in input {
            for r2 in input {
                if r1.id == r2.id {
                    continue;
                }

                if overlap(r1, r2) {
                    overlaps.insert(r1.id);
                    overlaps.insert(r2.id);
                }
            }
        }

        for r in input {
            if !overlaps.contains(&r.id) {
                return r.id;
            }
        }

        0
    }
}

#[derive(Debug)]
pub struct Rectangle {
    id: u32,
    left: u32,
    top: u32,
    width: u32,
    height: u32,
}

fn overlap(r1: &Rectangle, r2: &Rectangle) -> bool {
    r1.left < (r2.left + r2.width)
        && (r1.left + r1.width) > r2.left
        && r1.top < (r2.top + r2.height)
        && (r1.top + r1.height) > r2.top
}
