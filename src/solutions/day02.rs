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
        let mut two = 0;
        let mut three = 0;

        for id in input {
            let (has_two, has_three) = has_letters(id.as_str());
            if has_two {
                two += 1;
            }
            if has_three {
                three += 1;
            }
        }

        two * three
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

    let mut has_two = false;
    let mut has_three = false;
    for o in occurences.iter() {
        if o == &2 {
            has_two = true;
        }
        if o == &3 {
            has_three = true;
        }
    }

    (has_two, has_three)
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
