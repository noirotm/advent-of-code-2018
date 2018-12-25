use solver::Solver;
use std::collections::hash_map::DefaultHasher;
use std::fmt::Debug;
use std::fmt::Error;
use std::fmt::Formatter;
use std::fmt::Write;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::iter::repeat;
use std::hash::Hash;
use std::hash::Hasher;
use std::collections::HashMap;

pub struct Day18;

impl Solver for Day18 {
    type Input = Grid;
    type Output1 = u64;
    type Output2 = u64;

    fn day() -> u32 {
        18
    }

    fn parse_input<R: io::Read>(r: R) -> Grid {
        Grid::from_reader(r)
    }

    fn solve_first(input: &Grid) -> u64 {
        let mut g = input.clone();

        for _ in 0..10 {
            g = g.next_minute();
            //println!("{:?}", g);
        }

        let n_trees: u64 = g.cells.iter().flatten().map(|&b| if b == b'|' { 1 } else { 0 }).sum();
        let n_lumbs: u64 = g.cells.iter().flatten().map(|&b| if b == b'#' { 1 } else { 0 }).sum();

        n_trees * n_lumbs
    }

    fn solve_second(input: &Grid) -> u64 {
        let mut g = input.clone();
        let mut checksums = HashMap::new();

        // find the first pattern that gets repeated, and the repetition period
        let mut period = 0;
        let mut first_repeat_minute = 0;
        for minute in 1..=1000000000 {
            g = g.next_minute();
            //println!("{:?}", g);

            let mut s = DefaultHasher::new();
            g.hash(&mut s);
            let h = s.finish();

            if let Some(min) = checksums.get(&h) {
                period = minute - min;
                first_repeat_minute = *min;

                /*println!("First Minute: {}", first_repeat_minute);
                println!("Minute:       {}", minute);
                println!("Period:       {}", period);*/

                break;
            }

            checksums.insert(h, minute);
        }

        // warp to the last minute before 1000000000 that has this pattern
        let cycles = (1000000000 - first_repeat_minute) / period;
        let warp_to = first_repeat_minute + period * cycles;
        for minute in warp_to+1..=1000000000 {
            g = g.next_minute();
        }

        let n_trees: u64 = g.cells.iter().flatten().map(|&b| if b == b'|' { 1 } else { 0 }).sum();
        let n_lumbs: u64 = g.cells.iter().flatten().map(|&b| if b == b'#' { 1 } else { 0 }).sum();

        n_trees * n_lumbs
    }
}

#[derive(Clone, Hash)]
pub struct Grid {
    cells: Vec<Vec<u8>>,
}

impl Grid {
    fn from_reader<R: io::Read>(r: R) -> Self {
        let cells = BufReader::new(r)
            .lines()
            .filter_map(|l| l.ok())
            .map(|l| l.into_bytes())
            .collect();

        Self { cells }
    }

    fn next_minute(self) -> Grid {
        let height = self.cells.len();
        let width = self.cells[0].len();

        let mut cells: Vec<Vec<u8>> = repeat(repeat(b'.').take(width).collect())
            .take(height)
            .collect();

        for y in 0..self.cells.len() {
            for x in 0..self.cells[0].len() {
                let c = self.cells[y][x];
                let p = Pt { x, y };
                let nc = match c {
                    b'.' => self.next_open(&p),
                    b'|' => self.next_tree(&p),
                    b'#' => self.next_lumberyard(&p),
                    _ => panic!("should not be here"),
                };
                cells[y][x] = nc;
            }
        }

        Grid { cells }
    }

    fn next_open(&self, p: &Pt) -> u8 {
        let n_trees: usize = self
            .neighbours(p)
            .iter()
            .map(|&b| if b == b'|' { 1 } else { 0 })
            .sum();

        if n_trees >= 3 {
            b'|'
        } else {
            b'.'
        }
    }

    fn next_tree(&self, p: &Pt) -> u8 {
        let n_lumbs: usize = self
            .neighbours(p)
            .iter()
            .map(|&b| if b == b'#' { 1 } else { 0 })
            .sum();

        if n_lumbs >= 3 {
            b'#'
        } else {
            b'|'
        }
    }

    fn next_lumberyard(&self, p: &Pt) -> u8 {
        let n_trees: usize = self
            .neighbours(p)
            .iter()
            .map(|&b| if b == b'|' { 1 } else { 0 })
            .sum();
        let n_lumbs: usize = self
            .neighbours(p)
            .iter()
            .map(|&b| if b == b'#' { 1 } else { 0 })
            .sum();

        if n_trees >= 1 && n_lumbs >= 1 {
            b'#'
        } else {
            b'.'
        }
    }

    fn get(&self, x: usize, y: usize) -> Option<u8> {
        self.cells.get(y)?.get(x).cloned()
    }

    fn neighbours(&self, pt: &Pt) -> Vec<u8> {
        let height = self.cells.len();
        let width = self.cells[0].len();
        let mut v = vec![];

        if pt.x > 0 && pt.y > 0 {
            v.push(self.get(pt.x - 1, pt.y - 1));
        }
        if pt.y > 0 {
            v.push(self.get(pt.x, pt.y - 1));
        }
        if pt.x < width && pt.y > 0 {
            v.push(self.get(pt.x + 1, pt.y - 1));
        }
        if pt.x > 0 {
            v.push(self.get(pt.x - 1, pt.y));
        }
        if pt.x < width {
            v.push(self.get(pt.x + 1, pt.y));
        }
        if pt.x > 0 && pt.y < height {
            v.push(self.get(pt.x - 1, pt.y + 1));
        }
        if pt.y < height {
            v.push(self.get(pt.x, pt.y + 1));
        }
        if pt.x < width && pt.y < height {
            v.push(self.get(pt.x + 1, pt.y + 1));
        }

        v.iter().filter_map(|&v| v).collect()
    }
}

impl Debug for Grid {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        for row in self.cells.iter() {
            for b in row.iter() {
                f.write_char(*b as char)?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug)]
struct Pt {
    x: usize,
    y: usize,
}
