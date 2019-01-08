use crate::solver::Solver;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io;
use std::io::BufRead;
use std::io::BufReader;

pub struct Day06;

impl Solver for Day06 {
    type Input = Vec<Pt>;
    type Output1 = u64;
    type Output2 = u64;

    fn day(&self) -> u32 {
        6
    }

    fn parse_input<R: io::Read>(&self, r: R) -> Vec<Pt> {
        let r = BufReader::new(r);
        r.lines()
            .filter_map(|l| l.ok())
            .map(|line| {
                let mut s = line.split(", ");
                let x = s.next().unwrap().parse().unwrap();
                let y = s.next().unwrap().parse().unwrap();
                Pt { x, y }
            })
            .collect()
    }

    fn solve_first(&self, input: &Vec<Pt>) -> u64 {
        let min_x = input.iter().min_by_key(|e| e.x).unwrap().x;
        let min_y = input.iter().min_by_key(|e| e.y).unwrap().y;
        let max_x = input.iter().max_by_key(|e| e.x).unwrap().x;
        let max_y = input.iter().max_by_key(|e| e.y).unwrap().y;

        let mut areas = HashMap::new();
        let mut edge_ids = HashSet::new();

        for x in min_x..=max_x {
            for y in min_y..=max_y {
                let current_point = Pt { x, y };
                let dists = input
                    .iter()
                    .enumerate()
                    .map(|(i, pt)| (i, pt.dist(&current_point)))
                    .collect::<Vec<_>>();
                let &(id, min_dist) = dists.iter().min_by_key(|&&(_, s)| s).unwrap();
                let n_min_dist = dists.iter().filter(|&&(_, s)| s == min_dist).count();

                if n_min_dist == 1 {
                    let v = areas.get(&id).unwrap_or(&0) + 1;
                    areas.insert(id, v);

                    // edge detection
                    if x == min_x || x == max_x || y == min_y || y == max_y {
                        edge_ids.insert(id);
                    }
                }
            }
        }

        let (_, area) = areas
            .iter()
            .filter(|&(id, _)| !edge_ids.contains(id))
            .max_by_key(|&(_, &v)| v)
            .unwrap();

        *area as u64
    }

    fn solve_second(&self, input: &Vec<Pt>) -> u64 {
        let min_x = input.iter().min_by_key(|e| e.x).unwrap().x;
        let min_y = input.iter().min_by_key(|e| e.y).unwrap().y;
        let max_x = input.iter().max_by_key(|e| e.x).unwrap().x;
        let max_y = input.iter().max_by_key(|e| e.y).unwrap().y;

        let mut areas = HashMap::new();

        for x in min_x..=max_x {
            for y in min_y..=max_y {
                let current_point = Pt { x, y };
                let total_dist: i32 = input.iter().map(|pt| pt.dist(&current_point)).sum();

                if total_dist < 10000 {
                    areas.insert(current_point, total_dist);
                }
            }
        }

        areas.len() as u64
    }
}

#[derive(Debug, Eq, Hash)]
pub struct Pt {
    x: i32,
    y: i32,
}

impl PartialEq for Pt {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Pt {
    fn dist(&self, other: &Self) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}
