use crate::solver::Solver;
use std::{
    collections::HashSet,
    io::{self, BufRead, BufReader},
};

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<i64>;
    type Output1 = i64;
    type Output2 = i64;

    fn parse_input<R: io::Read>(&self, r: R) -> Vec<i64> {
        let r = BufReader::new(r);
        r.lines().flatten().flat_map(|l| l.parse()).collect()
    }

    fn solve_first(&self, input: &Vec<i64>) -> i64 {
        input.iter().sum()
    }

    fn solve_second(&self, input: &Vec<i64>) -> i64 {
        let mut frequencies = HashSet::new();
        let mut frequency = 0i64;

        frequencies.insert(0);

        for &v in input.iter().cycle() {
            frequency += v;
            if frequencies.contains(&frequency) {
                return frequency;
            } else {
                frequencies.insert(frequency);
            }
        }

        unreachable!()
    }
}
