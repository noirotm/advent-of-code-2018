use solver::Solver;
use std::io;

pub struct Day05;

impl Solver for Day05 {
    type Input = Vec<u8>;
    type Output1 = usize;
    type Output2 = usize;

    fn day() -> u32 {
        5
    }

    fn parse_input<R: io::Read>(mut r: R) -> Vec<u8> {
        let mut v = vec![];
        r.read_to_end(&mut v).expect("unable to read to vec");
        v
    }

    fn solve_first(input: &Vec<u8>) -> usize {
        size(input)
    }

    fn solve_second(input: &Vec<u8>) -> usize {
        let r = b'a'..=b'z';
        let mut min_val = input.len();
        for c in r {
            let v: Vec<_> = input
                .iter()
                .map(|b| *b)
                .filter(|ch| ch != &c && ch != &c.to_ascii_uppercase())
                .collect();
            let size = size(v.as_slice());
            if size < min_val {
                min_val = size;
            }
        }

        min_val
    }
}

fn size(input: &[u8]) -> usize {
    let mut pos = 0;
    let mut v = Vec::from(input);

    while pos < v.len() - 1 {
        let a = v[pos];
        let b = v[pos + 1];

        if (a as i8 - b as i8).abs() == 32 {
            v.remove(pos);
            v.remove(pos);
            pos = 0;
        } else {
            pos += 1;
        }
    }
    v.len()
}
