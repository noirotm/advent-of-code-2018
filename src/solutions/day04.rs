use chrono::prelude::*;
use regex::Regex;
use solver::Solver;
use std::collections::HashMap;
use std::io;
use std::io::BufRead;
use std::io::BufReader;

pub struct Day04;

impl Solver for Day04 {
    type Input = Vec<GuardEvent>;
    type Output1 = u32;
    type Output2 = u32;

    fn day() -> u32 {
        4
    }

    fn parse_input<R: io::Read>(r: R) -> Vec<GuardEvent> {
        let date_re = Regex::new(r"\[(.+)]").expect("Invalid regex");
        let shift_re = Regex::new(r"Guard #(\d+) begins shift").expect("Invalid regex");
        let sleep_re = Regex::new(r"falls asleep").expect("Invalid regex");
        //let wake_re = Regex::new(r"wakes up").expect("Invalid regex");

        let mut result = vec![];

        for line in BufReader::new(r).lines().filter_map(|l| l.ok()) {
            let date = date_re
                .captures(line.as_str())
                .unwrap()
                .get(1)
                .unwrap()
                .as_str();
            let date = Utc.datetime_from_str(date, "%Y-%m-%d %H:%M").unwrap();

            let event = if let Some(c) = shift_re.captures(line.as_str()) {
                Event::Shift(c.get(1).unwrap().as_str().parse().unwrap())
            } else if sleep_re.is_match(line.as_str()) {
                Event::Asleep
            } else {
                Event::Awake
            };

            result.push(GuardEvent { event, date });
        }

        result.sort_by(|a, b| a.date.cmp(&b.date));

        result
    }

    fn solve_first(input: &Vec<GuardEvent>) -> u32 {
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
                    if let None = guard_minutes.get(&current_id) {
                        guard_minutes.insert(current_id, HashMap::new());
                    }
                    let mut map = guard_minutes.get_mut(&current_id).unwrap();

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

    fn solve_second(input: &Vec<GuardEvent>) -> u32 {
        let mut current_id = 0;
        let mut last_asleep = None;
        let mut minute_guards: HashMap<u32, HashMap<u32, u32>> = HashMap::new();

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
                        if let None = minute_guards.get(&i) {
                            minute_guards.insert(i, HashMap::new());
                        }
                        let mut map = minute_guards.get_mut(&i).unwrap();

                        let n = map.get(&current_id).unwrap_or(&0) + 1;
                        map.insert(current_id, n);
                    }
                }
            }
        }

        let r = minute_guards
            .iter()
            .map(|map| (map.0, map.1.iter().max_by(|a, b| a.1.cmp(b.1)).unwrap()))
            .max_by(|a, b| (a.1).1.cmp((b.1).1))
            .unwrap();

        r.0 * (r.1).0
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
