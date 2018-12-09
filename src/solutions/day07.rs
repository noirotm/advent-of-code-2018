use regex::Regex;
use solver::Solver;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::iter::repeat_with;

pub struct Day07;

const WORKERS: usize = 5;
const TASK_DURATION: u8 = 60;

impl Solver for Day07 {
    type Input = Vec<(char, char)>;
    type Output1 = String;
    type Output2 = i32;

    fn day() -> u32 {
        7
    }

    fn parse_input<R: io::Read>(r: R) -> Vec<(char, char)> {
        let re = Regex::new(r"(.) must be finished before step (.)").expect("Invalid regex");

        BufReader::new(r)
            .lines()
            .filter_map(|l| l.ok())
            .filter_map(|s| {
                re.captures(s.as_str()).and_then(|c| {
                    Some((
                        c.get(2)?.as_str().chars().next()?,
                        c.get(1)?.as_str().chars().next()?,
                    ))
                })
            })
            .collect()
    }

    fn solve_first(input: &Vec<(char, char)>) -> String {
        let mut dependencies = build_dependency_graph(input);

        let len = dependencies.len();
        let mut result = Vec::with_capacity(len);

        while result.len() < len {
            let c = dependencies
                .iter()
                .filter(|&(_, v)| v.is_empty())
                .map(|(&k, _)| k)
                .min_by_key(|&c| c)
                .unwrap();

            for (_, dep) in dependencies.iter_mut() {
                dep.remove(&c);
            }

            dependencies.remove(&c);
            result.push(c);
        }

        result.iter().collect()
    }

    fn solve_second(input: &Vec<(char, char)>) -> i32 {
        let mut dependencies = build_dependency_graph(input);

        // create free workers
        let mut workers = repeat_with(|| Worker {
            task: None,
            progress: 0,
        })
        .take(WORKERS)
        .collect::<Vec<_>>();

        let mut seconds = -1;
        let len = dependencies.len();
        let mut n_done = 0;

        while n_done < len {
            // increment time for busy workers, mark task as done if time is reached
            for w in workers.iter_mut().filter(|w| w.task.is_some()) {
                let task = w.task.unwrap();

                // task is over
                if w.progress >= task_duration(task) {
                    for (_, dep) in dependencies.iter_mut() {
                        dep.remove(&task);
                    }

                    w.task = None;

                    n_done += 1;
                }

                w.progress += 1;
            }

            // identify available tasks
            let mut ready = dependencies
                .iter()
                .filter(|&(_, v)| v.is_empty())
                .map(|(&k, _)| k)
                .collect::<Vec<_>>();
            ready.sort();

            // affect available tasks to available workers
            for w in workers.iter_mut().filter(|w| w.task.is_none()) {
                if !ready.is_empty() {
                    w.task = Some(ready.remove(0));
                    w.progress = 1;

                    dependencies.remove(&w.task.unwrap());
                }
            }

            // next tick
            seconds += 1;
        }

        seconds
    }
}

fn build_dependency_graph(input: &Vec<(char, char)>) -> HashMap<char, HashSet<char>> {
    let mut dependencies = HashMap::new();

    // build dependency graph
    for (c, dep) in input {
        if !dependencies.contains_key(dep) {
            dependencies.insert(*dep, HashSet::new());
        }
        if !dependencies.contains_key(c) {
            dependencies.insert(*c, HashSet::new());
        }

        if let Some(deps) = dependencies.get_mut(c) {
            deps.insert(*dep);
        }
    }

    dependencies
}

struct Worker {
    task: Option<char>,
    progress: u32,
}

fn task_duration(v: char) -> u32 {
    (TASK_DURATION + (v as u8 - b'A') + 1) as u32
}
