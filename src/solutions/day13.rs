use crate::solver::Solver;
use std::cmp::Ordering;
use std::io;
use std::io::BufRead;
use std::io::BufReader;

pub struct Day13;

impl Solver for Day13 {
    type Input = RailSystem;
    type Output1 = String;
    type Output2 = String;

    fn day(&self) -> u32 {
        13
    }

    fn parse_input<R: io::Read>(&self, r: R) -> RailSystem {
        let mut tracks: Vec<Vec<u8>> = BufReader::new(r)
            .lines()
            .filter_map(|l| l.ok())
            .map(|s| s.into_bytes())
            .collect();
        let mut carts = vec![];

        for (y, t) in tracks.iter_mut().enumerate() {
            for (x, b) in t.iter_mut().enumerate() {
                let dir = match b {
                    b'>' => {
                        *b = b'-';
                        Some(Dir::Right)
                    }
                    b'v' => {
                        *b = b'|';
                        Some(Dir::Down)
                    }
                    b'<' => {
                        *b = b'-';
                        Some(Dir::Left)
                    }
                    b'^' => {
                        *b = b'|';
                        Some(Dir::Up)
                    }
                    _ => None,
                };
                if let Some(dir) = dir {
                    carts.push(Cart {
                        pos: Pos { x, y },
                        dir,
                        next_turn: Turn::Left,
                        deleted: false,
                    })
                }
            }
        }

        RailSystem { tracks, carts }
    }

    fn solve_first(&self, input: &RailSystem) -> String {
        let mut sys = input.clone();
        loop {
            let collisions = sys.step();
            if !collisions.is_empty() {
                return format!("{},{}", collisions[0].x, collisions[0].y);
            }
        }
    }

    fn solve_second(&self, input: &RailSystem) -> String {
        let mut sys = input.clone();
        loop {
            let _ = sys.step();

            if sys.carts.len() == 1 {
                return format!("{},{}", sys.carts[0].pos.x, sys.carts[0].pos.y);
            }
        }
    }
}

#[derive(Clone)]
pub struct RailSystem {
    tracks: Vec<Vec<u8>>,
    carts: Vec<Cart>,
}

impl RailSystem {
    #[allow(dead_code)]
    fn debug(&self) {
        let mut tracks = self.tracks.clone();
        for cart in self.carts.iter() {
            let c = match cart.dir {
                Dir::Right => b'>',
                Dir::Down => b'v',
                Dir::Left => b'<',
                Dir::Up => b'^',
            };
            tracks[cart.pos.y][cart.pos.x] = c;
        }
        for track in tracks {
            println!("{}", String::from_utf8(track).unwrap());
        }
    }

    fn step(&mut self) -> Vec<Pos> {
        let mut carts = self.carts.clone();

        // sort carts by x,y position
        carts.sort_by(|a, b| {
            let o = a.pos.x.cmp(&b.pos.x);
            if o == Ordering::Equal {
                a.pos.y.cmp(&b.pos.y)
            } else {
                o
            }
        });

        let mut deleted_pos = vec![];

        for i in 0..carts.len() {
            let mut cart = carts.get_mut(i).unwrap().clone();

            if cart.deleted {
                continue;
            }

            let t = self.tracks[cart.pos.y][cart.pos.x];
            cart.dir = match t {
                b'/' => cart.dir.mirror_normal(),
                b'\\' => cart.dir.mirror_reverse(),
                b'+' => {
                    let d = cart.next_turn.dir(&cart.dir);
                    cart.next_turn = cart.next_turn.next();
                    d
                }
                _ => cart.dir,
            };

            let mut new_pos = Pos {
                x: cart.pos.x,
                y: cart.pos.y,
            };

            match cart.dir {
                Dir::Down => {
                    new_pos.y += 1;
                }
                Dir::Up => {
                    new_pos.y -= 1;
                }
                Dir::Right => {
                    new_pos.x += 1;
                }
                Dir::Left => {
                    new_pos.x -= 1;
                }
            }

            // find out if we overwrite an existing point
            {
                let c = carts.iter_mut().find(|c| c.pos == new_pos);

                if let Some(c) = c {
                    // if we move over an existing point, we remove it
                    c.deleted = true;
                    deleted_pos.push(new_pos);
                    cart.deleted = true;
                } else {
                    cart.pos = new_pos;
                }
            }

            carts[i] = cart;
        }

        self.carts = carts
            .iter()
            .filter(|c| c.deleted == false)
            .map(|c| c.clone())
            .collect();

        // self.debug();
        deleted_pos
    }
}

#[derive(Clone)]
pub struct Cart {
    pos: Pos,
    dir: Dir,
    next_turn: Turn,
    deleted: bool,
}

#[derive(Copy, Clone)]
pub enum Dir {
    Right,
    Down,
    Left,
    Up,
}

impl Dir {
    fn mirror_normal(&self) -> Self {
        // handles the '/' turn
        match self {
            Dir::Right => Dir::Up,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Down,
            Dir::Up => Dir::Right,
        }
    }

    fn mirror_reverse(&self) -> Self {
        // handles the '\' turn
        match self {
            Dir::Right => Dir::Down,
            Dir::Down => Dir::Right,
            Dir::Left => Dir::Up,
            Dir::Up => Dir::Left,
        }
    }
}

#[derive(Eq, PartialEq, Hash, Clone)]
pub struct Pos {
    x: usize,
    y: usize,
}

#[derive(Clone)]
pub enum Turn {
    Left,
    Straight,
    Right,
}

impl Turn {
    fn next(&self) -> Self {
        match self {
            Turn::Left => Turn::Straight,
            Turn::Straight => Turn::Right,
            Turn::Right => Turn::Left,
        }
    }

    fn dir(&self, dir: &Dir) -> Dir {
        match (self, dir) {
            (Turn::Left, Dir::Left) => Dir::Down,
            (Turn::Left, Dir::Right) => Dir::Up,
            (Turn::Left, Dir::Up) => Dir::Left,
            (Turn::Left, Dir::Down) => Dir::Right,
            (Turn::Right, Dir::Left) => Dir::Up,
            (Turn::Right, Dir::Right) => Dir::Down,
            (Turn::Right, Dir::Up) => Dir::Right,
            (Turn::Right, Dir::Down) => Dir::Left,
            (Turn::Straight, Dir::Left) => Dir::Left,
            (Turn::Straight, Dir::Right) => Dir::Right,
            (Turn::Straight, Dir::Up) => Dir::Up,
            (Turn::Straight, Dir::Down) => Dir::Down,
        }
    }
}
