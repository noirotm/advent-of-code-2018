use crate::solver::Solver;
use regex::Regex;
use std::collections::VecDeque;
use std::fmt::Debug;
use std::fmt::Error;
use std::fmt::Formatter;
use std::fmt::Write;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::iter::repeat;

pub struct Day17;

impl Solver for Day17 {
    type Input = Vec<Pt>;
    type Output1 = u64;
    type Output2 = u64;

    fn day(&self) -> u32 {
        17
    }

    fn parse_input<R: io::Read>(&self, r: R) -> Vec<Pt> {
        coords_from_reader(r)
    }

    fn solve_first(&self, input: &Vec<Pt>) -> u64 {
        let bounds = coords_bounds(&input).unwrap();
        let mut grid = Grid::with_bounds(bounds);
        setup_grid(&mut grid, &input);

        let starting_point = Pt { x: 500, y: 0 };

        {
            let mut filler = GridFiller::new(&mut grid);
            filler.exec(&starting_point);
        }

        //println!("{:?}", grid);

        grid.cells
            .iter()
            .skip(grid.bounds.min_y)
            .flatten()
            .map(|&b| if b == b'|' || b == b'~' { 1 } else { 0 })
            .sum()
    }

    fn solve_second(&self, input: &Vec<Pt>) -> u64 {
        let bounds = coords_bounds(&input).unwrap();
        let mut grid = Grid::with_bounds(bounds);
        setup_grid(&mut grid, &input);

        let starting_point = Pt { x: 500, y: 0 };

        {
            let mut filler = GridFiller::new(&mut grid);
            filler.exec(&starting_point);
        }

        //println!("{:?}", grid);

        grid.cells
            .iter()
            .skip(grid.bounds.min_y)
            .flatten()
            .map(|&b| if b == b'~' { 1 } else { 0 })
            .sum()
    }
}

struct Grid {
    cells: Vec<Vec<u8>>,
    bounds: Bounds,
}

impl Grid {
    fn with_bounds(bounds: Bounds) -> Self {
        let width = bounds.max_x - bounds.min_x + 1;
        let height = bounds.max_y + 1;

        let cells = repeat(repeat(b'.').take(width).collect())
            .take(height)
            .collect();

        Self { cells, bounds }
    }

    fn set(&mut self, pt: &Pt, val: u8) {
        let x = pt.x.checked_sub(self.bounds.min_x).unwrap_or(0);
        self.cells[pt.y][x] = val;
    }

    fn get(&self, pt: &Pt) -> u8 {
        let x = pt.x.checked_sub(self.bounds.min_x).unwrap_or(0);
        self.cells[pt.y][x]
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

fn setup_grid(grid: &mut Grid, coords: &Vec<Pt>) {
    for p in coords {
        grid.set(p, b'#');
    }
}

enum Search {
    Down(Pt),
    Sides(Pt),
}

struct GridFiller<'a> {
    grid: &'a mut Grid,
    queue: VecDeque<Search>,
}

impl<'a> GridFiller<'a> {
    fn new(grid: &'a mut Grid) -> Self {
        Self {
            grid,
            queue: VecDeque::new(),
        }
    }

    fn exec(&mut self, starting_pt: &Pt) {
        self.queue.push_back(Search::Down(starting_pt.clone()));
        self.grid.set(&starting_pt, b'X');

        while let Some(s) = self.queue.pop_front() {
            match s {
                Search::Down(pt) => self.fill_down(&pt),
                Search::Sides(pt) => self.fill_sides(&pt),
            }
            //println!("{:?}", self.grid);
        }
    }

    fn fill_down(&mut self, pt: &Pt) {
        let mut pt = pt.clone();
        loop {
            // one step down
            pt = Pt {
                x: pt.x,
                y: pt.y + 1,
            };

            // stop if we are outside the grid
            if pt.y > self.grid.bounds.max_y {
                return;
            }

            let cell = self.grid.get(&pt);
            match cell {
                b'.' => {
                    // empty, let's waterize it
                    self.grid.set(&pt, b'|');
                }
                b'#' | b'~' => {
                    // clay or still water, we need to start searching left and right
                    self.queue.push_back(Search::Sides(Pt {
                        x: pt.x,
                        y: pt.y - 1,
                    }));
                    return;
                }
                _ => {
                    // if we're finding eg. '|' or '~' or nothing, just stop
                    return;
                }
            }
        }
    }

    fn fill_sides(&mut self, pt: &Pt) {
        // case our cell has been waterized already, go above
        if self.grid.get(pt) == b'~' {
            let above = Pt {
                x: pt.x,
                y: pt.y - 1,
            };
            self.queue.push_back(Search::Sides(above));
            return;
        }

        // loop left and right
        // while point under is water and next is empty
        let border_left = self.fill_side(pt, -1);
        let border_right = self.fill_side(pt, 1);

        // if we have both borders, we set all points in between to still water ~
        // and then do a fill sides for the point above
        // otherwise, do nothing
        match (border_left, border_right) {
            (Some(left), Some(right)) => {
                // make water still
                for x in left.x + 1..=right.x - 1 {
                    let pt = Pt { x, y: left.y };
                    self.grid.set(&pt, b'~');
                }

                let above = Pt {
                    x: pt.x,
                    y: pt.y - 1,
                };
                self.queue.push_back(Search::Sides(above));
            }
            _ => {
                self.grid.set(&pt, b'|');
            }
        }
    }

    fn fill_side(&mut self, pt: &Pt, dx: isize) -> Option<Pt> {
        // return border point if we are in a tank
        let mut pt = pt.clone();
        loop {
            pt = Pt {
                x: (pt.x as isize + dx) as usize,
                y: pt.y,
            };

            let pt_under = Pt {
                x: pt.x,
                y: pt.y + 1,
            };

            let cell = self.grid.get(&pt);
            let cell_under = self.grid.get(&pt_under);

            match (cell, cell_under) {
                (b'.', b'#') | (b'.', b'~') => {
                    // empty cell, has floor, waterize and continue
                    self.grid.set(&pt, b'|');
                }
                (b'#', _) => {
                    // stop here
                    return Some(pt);
                }
                (b'|', b'#') | (b'|', b'~') => {
                    // reached a spot with water, continue normally
                }
                (b'|', b'|') | (b'~', _) => {
                    // reached water, abort
                    return None;
                }
                (b'.', b'.') | (b'|', b'.') => {
                    // we're outside the tank, initialize a new down fill and quit
                    self.grid.set(&pt, b'|');
                    self.queue.push_back(Search::Down(pt));
                    return None;
                }
                (a, b) => {
                    println!("{:?}", self.grid);
                    println!("{:?}", pt);
                    panic!("we should not be here: {}, {}", a as char, b as char)
                }
            }
        }
    }
}

fn points_from_line(line: &str, (re_x, re_y): (&Regex, &Regex)) -> Option<Vec<Pt>> {
    if let Some(caps) = re_x.captures(line) {
        let x = caps.get(1)?.as_str().parse::<usize>().ok()?;
        let y1 = caps.get(2)?.as_str().parse::<usize>().ok()?;
        let y2 = caps.get(3)?.as_str().parse::<usize>().ok()?;
        Some((y1..=y2).into_iter().map(|y| Pt { x, y }).collect())
    } else if let Some(caps) = re_y.captures(line) {
        let y = caps.get(1)?.as_str().parse::<usize>().ok()?;
        let x1 = caps.get(2)?.as_str().parse::<usize>().ok()?;
        let x2 = caps.get(3)?.as_str().parse::<usize>().ok()?;
        Some((x1..=x2).into_iter().map(|x| Pt { x, y }).collect())
    } else {
        None
    }
}

fn coords_from_reader<R: io::Read>(r: R) -> Vec<Pt> {
    let re_x = Regex::new(r"x=(\d+), y=(\d+)\.\.(\d+)").unwrap();
    let re_y = Regex::new(r"y=(\d+), x=(\d+)\.\.(\d+)").unwrap();
    BufReader::new(r)
        .lines()
        .filter_map(|l| l.ok())
        .filter_map(|l| points_from_line(l.as_str(), (&re_x, &re_y)))
        .flatten()
        .collect()
}

fn coords_bounds(coords: &Vec<Pt>) -> Option<Bounds> {
    Some(Bounds {
        min_x: coords.iter().map(|p| p.x).min()? - 1,
        min_y: coords.iter().map(|p| p.y).min()?,
        max_x: coords.iter().map(|p| p.x).max()? + 1,
        max_y: coords.iter().map(|p| p.y).max()?,
    })
}

#[derive(Debug)]
struct Bounds {
    min_x: usize,
    min_y: usize,
    max_x: usize,
    max_y: usize,
}

#[derive(Clone, Debug)]
pub struct Pt {
    x: usize,
    y: usize,
}
