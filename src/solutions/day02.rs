use solver::read_to_vec;
use solver::Solver;
use std::cmp::min;
use std::io;
use std::io::BufRead;
use std::io::BufReader;

pub struct Day02;

impl Solver for Day02 {
    type Input = Vec<String>;
    type Output1 = u64;
    type Output2 = String;

    fn day() -> u32 {
        2
    }

    fn parse_input<R: io::Read>(r: R) -> io::Result<Vec<String>> {
        Ok(read_to_vec(r))
    }

    fn solve_first(input: &Vec<String>) -> u64 {
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

    fn solve_second(input: &Vec<String>) -> String {
        // quadratic complexity :(
        for s1 in input {
            for s2 in input {
                if differ_by_one_character(s1.as_str(), s2.as_str()) {
                    return common_letters(s1, s2);
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

fn differ_by_one_character(s1: &str, s2: &str) -> bool {
    let mut diffs = 0;
    let l = min(s1.len(), s2.len());

    for c in 0..l {
        if s1.as_bytes()[c] != s2.as_bytes()[c] {
            diffs += 1;
        }
        if diffs > 1 {
            return false;
        }
    }

    diffs == 1
}

fn common_letters(s1: &str, s2: &str) -> String {
    let l = min(s1.len(), s2.len());
    let mut s = String::new();

    for c in 0..l {
        if s1.as_bytes()[c] == s2.as_bytes()[c] {
            s.push_str(s1[c..c + 1].as_ref())
        }
    }

    s
}
