use crate::solver::Solver;
use regex::Regex;
use std::io;

pub struct Day09;

impl Solver for Day09 {
    type Input = (usize, usize);
    type Output1 = u64;
    type Output2 = u64;

    fn parse_input<R: io::Read>(&self, mut r: R) -> (usize, usize) {
        let re =
            Regex::new(r"(\d+) players; last marble is worth (\d+) points").expect("bad regex");
        let mut s = String::new();
        r.read_to_string(&mut s).expect("unable to read to string");

        let captures = re.captures(s.as_str()).unwrap();
        let players = captures.get(1).unwrap().as_str().parse().unwrap();
        let marbles = captures.get(2).unwrap().as_str().parse().unwrap();

        (players, marbles)
    }

    fn solve_first(&self, &(num_players, num_marbles): &(usize, usize)) -> u64 {
        let mut players = vec![0u64; num_players];
        let mut circle = Circle::with_capacity(num_marbles + 1);

        for (marble, player) in (1..=num_marbles)
            .into_iter()
            .zip((0..num_players).into_iter().cycle())
        {
            if marble % 23 == 0 {
                let val = circle.remove();
                let score = players.get_mut(player).unwrap();
                *score += (val + marble) as u64;
            } else {
                circle.insert_next(marble);
            }
        }

        *players.iter().max_by_key(|&&e| e).unwrap()
    }

    fn solve_second(&self, &(num_players, num_marbles): &(usize, usize)) -> u64 {
        Self::solve_first(self, &(num_players, num_marbles * 100))
    }
}

struct Circle {
    marbles: Vec<usize>,
    current_idx: usize,
}

impl Circle {
    fn with_capacity(cap: usize) -> Self {
        let mut marbles = Vec::with_capacity(cap);
        marbles.push(0);
        Self {
            marbles,
            current_idx: 0,
        }
    }

    fn insert_next(&mut self, marble: usize) {
        let mut next_idx = self.current_idx;

        for _ in 0..2 {
            next_idx += 1;
            if next_idx > self.marbles.len() {
                next_idx = 1;
            }
        }

        self.marbles.insert(next_idx, marble);
        self.current_idx = next_idx;
    }

    fn remove(&mut self) -> usize {
        let mut prev_idx = self.current_idx;

        for _ in 0..7 {
            if prev_idx == 0 {
                prev_idx = self.marbles.len() - 1;
            } else {
                prev_idx -= 1;
            }
        }

        let value = self.marbles.remove(prev_idx);

        let mut next_idx = prev_idx;
        if next_idx > self.marbles.len() {
            next_idx = 0;
        }
        self.current_idx = next_idx;

        value
    }
}
