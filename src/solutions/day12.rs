use solver::Solver;
use std::cmp::max;
use std::collections::HashMap;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Seek;
use std::io::SeekFrom;

pub struct Day12;

impl Solver for Day12 {
    type Input = LifeInput;
    type Output1 = i64;
    type Output2 = i64;

    fn day() -> u32 {
        12
    }

    fn parse_input<R: io::Read + io::Seek>(r: R) -> LifeInput {
        let mut r = BufReader::new(r);

        r.seek(SeekFrom::Current(15)).expect("unable to seek");

        let mut state_str = String::new();
        r.read_line(&mut state_str)
            .expect("unable to read state line");
        state_str.pop();

        let mut empty_line = String::new();
        r.read_line(&mut empty_line)
            .expect("unable to read empty line");

        let map = r
            .lines()
            .filter_map(|s| s.ok())
            .filter_map(|s| {
                let b = s.as_bytes();
                let from = Vec::from(&b[0..5]);
                let to = b[9];

                Some((from, to))
            }).collect();

        LifeInput {
            state: State {
                pots: state_str.into_bytes(),
                initial_index: 0,
            },
            rules: map,
        }
    }

    fn solve_first(input: &LifeInput) -> i64 {
        solve_for_generations(&input, 20)
    }

    fn solve_second(input: &LifeInput) -> i64 {
        solve_for_generations(&input, 50000000000)
    }
}

#[derive(Debug)]
pub struct LifeInput {
    state: State,
    rules: HashMap<Vec<u8>, u8>,
}

#[derive(Debug)]
pub struct State {
    pots: Vec<u8>,
    initial_index: i32,
}

fn next_state(current_state: &State, rules: &HashMap<Vec<u8>, u8>) -> State {
    let first_alive_pos = current_state.pots.iter().position(|&e| e == b'#').unwrap();
    let last_alive_pos = current_state.pots.iter().rposition(|&e| e == b'#').unwrap();
    let left_extension = max(0, 2 - first_alive_pos as isize) as usize;
    let right_extension = max(0, 2 + last_alive_pos as isize) as usize;

    let mut output = State {
        pots: vec![b'.'; left_extension + right_extension],
        initial_index: current_state.initial_index - left_extension as i32,
    };

    for (i, p) in output.pots.iter_mut().enumerate() {
        let i = i as isize - left_extension as isize;
        let from = vec![
            *vec_entry(&current_state.pots, i - 2, &b'.'),
            *vec_entry(&current_state.pots, i - 1, &b'.'),
            *vec_entry(&current_state.pots, i, &b'.'),
            *vec_entry(&current_state.pots, i + 1, &b'.'),
            *vec_entry(&current_state.pots, i + 2, &b'.'),
        ];
        let to = rules.get(&from).unwrap_or(&b'.');
        *p = *to;
    }

    output
}

fn vec_entry<'a, T>(v: &'a Vec<T>, i: isize, def: &'a T) -> &'a T {
    if i < 0 {
        &def
    } else {
        v.get(i as usize).unwrap_or(def)
    }
}

fn solve_for_generations(input: &LifeInput, generations: u64) -> i64 {
    let mut state = State {
        pots: input.state.pots.clone(),
        initial_index: input.state.initial_index,
    };
    println!("0 {}", String::from_utf8(state.pots.clone()).unwrap());

    for i in 1..=generations {
        state = next_state(&state, &input.rules);

        println!("{} {}", i, String::from_utf8(state.pots.clone()).unwrap());


    }

    state
        .pots
        .iter()
        .enumerate()
        .filter(|&(i, &c)| c == b'#')
        .map(|(i, _)| i as i64 + state.initial_index as i64)
        .sum()
}
