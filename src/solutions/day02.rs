use crate::solver::read_to_vec;
use crate::solver::Solver;
use std::io;

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<String>;
    type Output1 = u64;
    type Output2 = String;

    fn parse_input<R: io::Read>(&self, r: R) -> Vec<String> {
        read_to_vec(r)
    }

    fn solve_first(&self, input: &Vec<String>) -> u64 {
        let (s2, s3) = input
            .iter()
            .map(|s| has_letters(s.as_str()))
            .map(|(h1, h2)| (h1 as u64, h2 as u64))
            .fold((0u64, 0u64), |(h2, h3), (a, b)| (h2 + a, h3 + b));

        s2 * s3
    }

    fn solve_second(&self, input: &Vec<String>) -> String {
        // quadratic complexity :(
        for s1 in input {
            for s2 in input {
                if let Some(s) = common_letters(s1, s2) {
                    return s;
                }
            }
        }

        String::new()
    }
}

fn has_letters(s: &str) -> (bool, bool) {
    let mut occurences = [0; 26];
    for c in s.bytes() {
        let idx = c - b'a';
        occurences[idx as usize] += 1;
    }

    occurences
        .iter()
        .map(|&o| (o == 2, o == 3))
        .fold((false, false), |(h2, h3), (a, b)| (h2 || a, h3 || b))
}

fn common_letters(s1: &str, s2: &str) -> Option<String> {
    let mut diffs = 0;
    let mut s = String::new();

    for (c1, c2) in s1.chars().zip(s2.chars()) {
        if c1 == c2 {
            s.push(c1);
        } else {
            diffs += 1;
            if diffs > 1 {
                return None;
            }
        }
    }

    if diffs == 1 {
        Some(s)
    } else {
        None
    }
}
