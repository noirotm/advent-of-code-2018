use crate::solver::Solver;
use regex::Regex;
use std::io;
use std::io::BufRead;
use std::io::BufReader;

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<PointEntry>;
    type Output1 = String;
    type Output2 = i32;

    fn parse_input<R: io::Read>(&self, r: R) -> Vec<PointEntry> {
        let re = Regex::new(r"position=<\s*(-?\d+),\s*(-?\d+)> velocity=<\s*(-?\d+),\s*(-?\d+)>")
            .expect("bad regex");

        BufReader::new(r)
            .lines()
            .filter_map(|l| l.ok())
            .filter_map(|s| {
                re.captures(s.as_str()).and_then(|c| {
                    Some(PointEntry {
                        pos: Coords {
                            x: c.get(1)?.as_str().parse().ok()?,
                            y: c.get(2)?.as_str().parse().ok()?,
                        },
                        velocity: Coords {
                            x: c.get(3)?.as_str().parse().ok()?,
                            y: c.get(4)?.as_str().parse().ok()?,
                        },
                    })
                })
            })
            .collect()
    }

    fn solve_first(&self, input: &Vec<PointEntry>) -> String {
        let mut cur_state = next_state(input);

        loop {
            let next_state = next_state(&cur_state);
            let next_entropy = entropy(&next_state);

            if next_entropy < 70 {
                print_grid(&next_state);
                break;
            }

            cur_state = next_state;
        }

        "See output".to_string()
    }

    fn solve_second(&self, input: &Vec<PointEntry>) -> i32 {
        let mut cur_state = next_state(input);
        let mut time = 1;

        loop {
            time += 1;

            let next_state = next_state(&cur_state);
            let next_entropy = entropy(&next_state);

            if next_entropy < 70 {
                break;
            }

            cur_state = next_state;
        }

        time
    }
}

#[derive(Debug)]
pub struct PointEntry {
    pos: Coords,
    velocity: Coords,
}

#[derive(Debug)]
pub struct Coords {
    x: i32,
    y: i32,
}

fn next_state(points: &Vec<PointEntry>) -> Vec<PointEntry> {
    points
        .iter()
        .map(|e| PointEntry {
            pos: Coords {
                x: e.pos.x + e.velocity.x,
                y: e.pos.y + e.velocity.y,
            },
            velocity: Coords {
                x: e.velocity.x,
                y: e.velocity.y,
            },
        })
        .collect()
}

fn entropy(points: &Vec<PointEntry>) -> u64 {
    let mut v = points.iter().map(|p| p.pos.x).collect::<Vec<_>>();
    v.sort();
    v.dedup();
    let s_x = v.len();

    let mut v = points.iter().map(|p| p.pos.y).collect::<Vec<_>>();
    v.sort();
    v.dedup();
    let s_y = v.len();

    s_x as u64 + s_y as u64
}

fn print_grid(points: &Vec<PointEntry>) {
    let min_x = points.iter().min_by_key(|e| e.pos.x).unwrap().pos.x;
    let min_y = points.iter().min_by_key(|e| e.pos.y).unwrap().pos.y;
    let max_x = points.iter().max_by_key(|e| e.pos.x).unwrap().pos.x;
    let max_y = points.iter().max_by_key(|e| e.pos.y).unwrap().pos.y;

    let mut out = String::new();
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let c = if points.iter().any(|p| p.pos.x == x && p.pos.y == y) {
                '◻'
            } else {
                ' '
            };
            out.push(c);
        }
        out.push('\n');
    }

    print!("{}", out);
}
