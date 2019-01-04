use crate::solver::Solver;
use std::collections::VecDeque;
use std::io;
use std::io::BufReader;
use std::io::Read;

pub struct Day20 {}

impl Solver for Day20 {
    type Input = Directions;
    type Output1 = u64;
    type Output2 = u64;

    fn day() -> u32 {
        20
    }

    fn parse_input<R: io::Read>(r: R) -> Directions {
        Directions::from_reader(r)
    }

    fn solve_first(input: &Directions) -> u64 {
        input.max_len()
    }

    fn solve_second(_input: &Directions) -> u64 {
        //input.n_further_from(10)
        0
    }
}

#[derive(Clone, Debug)]
pub enum Dir {
    N,
    S,
    E,
    W,
}

#[derive(Debug)]
pub enum Directions {
    Seq(Vec<Directions>),
    Choice(Vec<Directions>),
    Dir(Dir),
}

impl Directions {
    fn from_reader<R: io::Read>(r: R) -> Directions {
        let mut bytes = BufReader::new(r).bytes();
        Self::parse_choice(&mut bytes)
    }

    /*fn to_tree(&self) -> Directions {
    let mut queue = VecDeque::new();
    queue.push_back(vec![0usize]);

    let mut tree = vec![];
    let mut current_branch = &mut tree;

    while let Some(idx) = queue.pop_front() {
        let d = self.nested_get(&idx);
        match d {
            /*Some(Directions::Dir(dir)) => {
                current_branch.push(Directions::Dir(dir.clone()));
            }*/
    Some(Directions::Seq(seq)) => {
    for (i, e) in seq.iter().enumerate() {
    match e {
    Directions::Dir(dir) => {
    current_branch.push(Directions::Dir(dir.clone()))
    }
    Directions::Choice(_) => {
    // push choice index
    let mut choice_idx = idx.clone();
    choice_idx.push(i);
    queue.push_back(choice_idx);

    break; // break because each new choice means a new iteration will take place
    }
    _ => panic!("seq of seqs"),
    }
    }
    }
    Some(Directions::Choice(choice)) => {
    //

    for (i, e) in choice.iter().enumerate() {}
    }
    _ => panic!("should not point on a dir"),
    }
    }

    Directions::Seq(tree)
    }*/

    fn nested_get(&self, idx: &[usize]) -> Option<&Directions> {
        let mut d = self;
        for &i in idx {
            d = match d {
                Directions::Seq(s) => s.get(i)?,
                Directions::Choice(c) => c.get(i)?,
                Directions::Dir(_) => d,
            };
        }

        Some(d)
    }

    fn parse_choice<E>(bytes: &mut impl Iterator<Item = Result<u8, E>>) -> Directions {
        let mut seq = vec![];
        let mut choice = vec![];

        loop {
            match bytes.next().and_then(|b| b.ok()) {
                Some(b'N') => seq.push(Directions::Dir(Dir::N)),
                Some(b'E') => seq.push(Directions::Dir(Dir::E)),
                Some(b'S') => seq.push(Directions::Dir(Dir::S)),
                Some(b'W') => seq.push(Directions::Dir(Dir::W)),
                Some(b'(') => seq.push(Self::parse_choice(bytes)),
                Some(b')') => {
                    break;
                }
                Some(b'|') => {
                    choice.push(Directions::Seq(seq));
                    seq = vec![];
                }
                Some(_) => {}
                None => break,
            }
        }

        choice.push(Directions::Seq(seq));
        Directions::Choice(choice)
    }

    fn max_len(&self) -> u64 {
        match self {
            Directions::Dir(_) => 1,
            Directions::Seq(seq) => Self::seq_max_len(&seq),
            Directions::Choice(choice) => Self::choice_max_len(&choice),
        }
    }

    fn seq_max_len(seq: &Vec<Directions>) -> u64 {
        if Self::is_detour(seq) {
            return 0;
        }
        seq.iter().map(|d| d.max_len()).sum()
    }

    fn choice_max_len(choice: &Vec<Directions>) -> u64 {
        choice.iter().map(|d| d.max_len()).max().unwrap_or(0)
    }

    // if a sequence goes back to its origin, it is a detour
    fn is_detour(seq: &Vec<Directions>) -> bool {
        let sum = seq
            .iter()
            .filter_map(|d| match d {
                Directions::Dir(Dir::N) => Some((0, -1)),
                Directions::Dir(Dir::S) => Some((0, 1)),
                Directions::Dir(Dir::E) => Some((1, 0)),
                Directions::Dir(Dir::W) => Some((-1, 0)),
                _ => None,
            })
            .fold((0, 0), |(ax, ay), (bx, by)| (ax + bx, ay + by));

        sum == (0, 0)
    }

    /*fn n_further_from(&self, cost: i32) -> u64 {
        match self {
            Directions::North | Directions::South | Directions::East | Directions::West => {
                if cost <= 0 {
                    1
                } else {
                    0
                }
            }
            Directions::Seq(seq) => Self::seq_n_further_from(&seq, cost),
            Directions::Choice(choice) => Self::choice_n_further_from(&choice, cost),
        }
    }

    fn seq_n_further_from(seq: &Vec<Directions>, cost: i32) -> u64 {
        if Self::is_detour(seq) {
            return match cost {
                x if x <= 1 => 2,
                2 => 1,
                _ => 0,
            };
        } // wrong because a detour can be more than 4 characters (eg. NNEWSS)

        let mut n = 0;
        let mut cost = cost;
        for d in seq {
            match d {
                Directions::North | Directions::South | Directions::East | Directions::West => {
                    cost -= 1;
                    if cost <= 0 {
                        n += 1;
                    }
                }
                Directions::Seq(seq) => {
                    n += Self::seq_n_further_from(&seq, cost);
                }
                Directions::Choice(choice) => {
                    n += Self::choice_n_further_from(&choice, cost);
                }
            }
        }

        n
    }

    fn choice_n_further_from(choice: &Vec<Directions>, cost: i32) -> u64 {
        let mut n = 0;
        for d in choice {
            match d {
                Directions::North | Directions::South | Directions::East | Directions::West => {
                    if cost <= 0 {
                        n += 1;
                    }
                }
                Directions::Seq(seq) => {
                    n += Self::seq_n_further_from(&seq, cost);
                }
                Directions::Choice(choice) => {
                    n += Self::choice_n_further_from(&choice, cost);
                }
            }
        }

        n
    }*/
}
