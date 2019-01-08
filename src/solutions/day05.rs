use crate::solver::Solver;
use std::io;

pub struct Day05;

impl Solver for Day05 {
    type Input = Vec<u8>;
    type Output1 = usize;
    type Output2 = usize;

    fn parse_input<R: io::Read>(&self, mut r: R) -> Vec<u8> {
        let mut v = vec![];
        r.read_to_end(&mut v).expect("unable to read to vec");
        v
    }

    fn solve_first(&self, input: &Vec<u8>) -> usize {
        size(input)
    }

    fn solve_second(&self, input: &Vec<u8>) -> usize {
        (b'a'..=b'z')
            .map(|c| {
                let v = input
                    .iter()
                    .filter(|&&ch| ch != c && ch != c.to_ascii_uppercase())
                    .map(|b| *b)
                    .collect::<Vec<_>>();
                size(&v)
            })
            .min_by_key(|e| *e)
            .unwrap()
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

            pos = if pos > 0 { pos - 1 } else { 0 };
        } else {
            pos += 1;
        }
    }
    v.len()
}
