use solver::Solver;
use std::io;

pub struct Day14;

impl Solver for Day14 {
    type Input = Vec<u8>;
    type Output1 = String;
    type Output2 = usize;

    fn day() -> u32 {
        14
    }

    fn parse_input<R: io::Read>(r: R) -> Vec<u8> {
        r.bytes()
            .filter_map(|b| b.ok())
            .filter_map(|b| char::from(b).to_digit(10))
            .map(|d| d as u8)
            .collect()
    }

    fn solve_first(input: &Vec<u8>) -> String {
        let n = recipes_str(input)
            .parse::<usize>()
            .expect("invalid integer");

        let mut elves = [0usize, 1usize];

        let mut recipes = vec![3, 7];

        loop {
            // compute next recipes
            let sum = elves.iter().map(|&e| recipes[e]).sum::<u8>();
            let mut digits = sum
                .to_string()
                .chars()
                .filter_map(|c| c.to_digit(10))
                .map(|i| i as u8)
                .collect();
            recipes.append(&mut digits);

            // move elves forward
            for elf in elves.iter_mut() {
                let mut new_idx = *elf + recipes[*elf] as usize + 1;
                new_idx %= recipes.len();
                *elf = new_idx;
            }

            if recipes.len() > n + 10 {
                let r = &recipes[n..n + 10];
                return recipes_str(r)
            }
        }
    }

    fn solve_second(input: &Vec<u8>) -> usize {
        let n = recipes_str(input)
            .parse::<usize>()
            .expect("invalid integer");

        let mut elves = [0usize, 1usize];

        let mut recipes = vec![3, 7];

        loop {
            // compute next recipes
            let sum = elves.iter().map(|&e| recipes[e]).sum::<u8>();
            let mut digits = sum
                .to_string()
                .chars()
                .filter_map(|c| c.to_digit(10))
                .map(|i| i as u8)
                .collect();
            recipes.append(&mut digits);

            // move elves forward
            for elf in elves.iter_mut() {
                let mut new_idx = *elf + recipes[*elf] as usize + 1;
                new_idx %= recipes.len();
                *elf = new_idx;
            }

            // look at the N last digits
            if recipes.len() >= input.len() {
                let tail = &recipes[recipes.len() - input.len()..recipes.len()];

                if tail.iter().eq(input.iter()) {
                    return recipes.len() - input.len();
                }
            }

            // look at N last digits, minus 1 because we can have added 2 recipes
            if recipes.len() > input.len() {
                let tail = &recipes[recipes.len() - input.len() - 1..recipes.len() - 1];

                if tail.iter().eq(input.iter()) {
                    return recipes.len() - input.len() - 1;
                }
            }
        }
    }
}

fn recipes_str(recipes: &[u8]) -> String {
    let s = recipes.iter().map(|b| b.to_string()).collect::<Vec<_>>();
    s.join("")
}
