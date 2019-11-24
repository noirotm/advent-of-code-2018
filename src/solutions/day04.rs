use crate::solver::Solver;
use chrono::prelude::*;
use regex::Regex;
use std::{
    collections::HashMap,
    io::{self, BufRead, BufReader},
};

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<GuardEvent>;
    type Output1 = u32;
    type Output2 = u32;

    fn parse_input<R: io::Read>(&self, r: R) -> Vec<GuardEvent> {
        let date_re = Regex::new(r"\[(.+)]").expect("Invalid regex");
        let shift_re = Regex::new(r"Guard #(\d+) begins shift").expect("Invalid regex");
        let sleep_re = Regex::new(r"falls asleep").expect("Invalid regex");
        //let wake_re = Regex::new(r"wakes up").expect("Invalid regex");

        let mut result = BufReader::new(r)
            .lines()
            .flatten()
            .flat_map(|s| {
                let date = date_re.captures(s.as_str())?.get(1)?.as_str();
                let date = Utc.datetime_from_str(date, "%Y-%m-%d %H:%M").ok()?;
                let event = if let Some(c) = shift_re.captures(s.as_str()) {
                    Event::Shift(c.get(1).unwrap().as_str().parse().unwrap())
                } else if sleep_re.is_match(s.as_str()) {
                    Event::Asleep
                } else {
                    Event::Awake
                };
                Some(GuardEvent { event, date })
            })
            .collect::<Vec<_>>();

        result.sort_by(|a, b| a.date.cmp(&b.date));

        result
    }

    fn solve_first(&self, input: &Vec<GuardEvent>) -> u32 {
        let mut current_id = 0;
        let mut last_asleep = None;
        let mut asleep = HashMap::new();
        let mut guard_minutes: HashMap<u32, HashMap<u32, u32>> = HashMap::new();

        let mut max_minutes = 0;
        let mut max_id = 0;

        for evt in input {
            match evt.event {
                Event::Shift(id) => {
                    current_id = id;
                }
                Event::Asleep => {
                    last_asleep = Some(evt.date);
                }
                Event::Awake => {
                    let duration = evt.date.signed_duration_since(last_asleep.unwrap());
                    let minutes = duration.num_minutes();

                    let id_asleep = asleep.get(&current_id).unwrap_or(&0) + minutes - 1;
                    asleep.insert(current_id, id_asleep);

                    if id_asleep > max_minutes {
                        max_minutes = id_asleep;
                        max_id = current_id;
                    }

                    let minutes_range = last_asleep.unwrap().minute()..evt.date.minute();
                    guard_minutes.entry(current_id).or_insert_with(HashMap::new);
                    let map = guard_minutes.get_mut(&current_id).unwrap();

                    for i in minutes_range {
                        let n = map.get(&i).unwrap_or(&0) + 1;
                        map.insert(i, n);
                    }
                }
            }
        }

        let minutes_map = guard_minutes.get(&max_id).expect("expected a map");

        let max = minutes_map
            .iter()
            .max_by(|a, b| a.1.cmp(b.1))
            .expect("expected a max");

        max_id * max.0
    }

    fn solve_second(&self, input: &Vec<GuardEvent>) -> u32 {
        let mut current_id = 0;
        let mut last_asleep = None;
        let mut minute_guards = HashMap::new();

        for evt in input {
            match evt.event {
                Event::Shift(id) => {
                    current_id = id;
                }
                Event::Asleep => {
                    last_asleep = Some(evt.date);
                }
                Event::Awake => {
                    let minutes_range = last_asleep.unwrap().minute()..evt.date.minute();

                    for i in minutes_range {
                        minute_guards.entry(i).or_insert_with(HashMap::new);
                        let map = minute_guards.get_mut(&i).unwrap();

                        let n = map.get(&current_id).unwrap_or(&0) + 1;
                        map.insert(current_id, n);
                    }
                }
            }
        }

        let (minute, (guard, _)) = minute_guards
            .iter()
            .map(|(minute, guards)| {
                (
                    minute,
                    guards.iter().max_by(|(_, a), (_, b)| a.cmp(b)).unwrap(),
                )
            })
            .max_by(|(_, (_, a)), (_, (_, b))| a.cmp(b))
            .unwrap();

        minute * guard
    }
}

#[derive(Debug)]
enum Event {
    Shift(u32),
    Asleep,
    Awake,
}

#[derive(Debug)]
pub struct GuardEvent {
    event: Event,
    date: DateTime<Utc>,
}
