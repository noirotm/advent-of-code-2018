use crate::solver::Solver;
use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::collections::VecDeque;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::iter::repeat;
use std::usize::MAX;

pub struct Problem;

impl Solver for Problem {
    type Input = Map;
    type Output1 = i64;
    type Output2 = i64;

    fn parse_input<R: io::Read>(&self, r: R) -> Map {
        Map::from_reader(r)
    }

    fn solve_first(&self, input: &Map) -> i64 {
        let mut map = input.clone();
        loop {
            let status = map.round();
            //map.debug();

            if let Status::Win(t) = status {
                let units = map.find_targets_of_type(t);

                let hp = units.iter().map(|u| u.unit.hp).sum::<usize>();

                println!("{} x {} = {}", hp, map.rounds, hp * map.rounds);

                return hp as i64 * map.rounds as i64;
            }
        }
    }

    fn solve_second(&self, input: &Map) -> i64 {
        let mut elf_atk = 4;
        loop {
            let mut map = input.clone();
            map.elf_atk = elf_atk;

            let start_elves = map.find_targets_of_type(UnitType::Elf).len();

            loop {
                let status = map.round();
                //map.debug();

                // quit as soon as an elf dies
                let remaining_elves = map.find_targets_of_type(UnitType::Elf);
                if remaining_elves.len() < start_elves {
                    break;
                }

                if let Status::Win(UnitType::Elf) = status {
                    let hp = remaining_elves.iter().map(|u| u.unit.hp).sum::<usize>();

                    println!("Attack power: {}", elf_atk);
                    println!("start elves: {} --> {}", start_elves, remaining_elves.len());

                    if remaining_elves.len() == start_elves {
                        println!("{} x {} = {}", hp, map.rounds, hp * map.rounds);

                        return hp as i64 * map.rounds as i64;
                    }

                    break;
                }
            }
            elf_atk += 1;
        }
    }
}

#[derive(Clone)]
pub struct Map {
    cells: Vec<Vec<Elem>>,
    dims: (usize, usize),
    rounds: usize,
    elf_atk: usize,
}

impl Map {
    fn from_reader<R: io::Read>(r: R) -> Self {
        let r = BufReader::new(r);
        let cells = r
            .lines()
            .filter_map(|l| l.ok())
            .map(|l| {
                l.chars()
                    .filter_map(|c| Elem::from_char(c))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let h = cells.len();
        let w = cells.first().map_or(0, |c| c.len());

        Map {
            cells,
            dims: (w, h),
            rounds: 0,
            elf_atk: 3,
        }
    }

    #[allow(dead_code)]
    fn debug(&self) {
        let (w, h) = self.dims;
        for y in 0..h {
            for x in 0..w {
                let elem = self.elem(x, y);
                let c = match elem {
                    Elem::Unit(u) => match u.unit_type {
                        UnitType::Elf => 'E',
                        UnitType::Goblin => 'G',
                    },
                    Elem::Open => '.',
                    Elem::Wall => '#',
                };
                print!("{}", c);
            }
            println!();
        }

        for elf in self.find_targets_of_type(UnitType::Elf).iter() {
            print!("E({}, {}) ", elf.unit.hp, elf.unit.rounds);
        }
        println!();
        for goblin in self.find_targets_of_type(UnitType::Goblin).iter() {
            print!("G({}, {}) ", goblin.unit.hp, goblin.unit.rounds);
        }
        println!();

        println!("Round: {}\n", self.rounds);

        //std::io::stdin().bytes().next();
    }

    fn round(&mut self) -> Status {
        let (w, h) = self.dims;
        for y in 0..h {
            for x in 0..w {
                let elem = self.elem(x, y);

                if let Some(unit) = elem.unit() {
                    let unit_pos = UnitPos {
                        unit,
                        pos: Pos { x, y },
                    };

                    let status = self.turn(unit_pos);
                    if status != Status::Continue {
                        return status;
                    }
                }
            }
        }

        self.rounds += 1;
        Status::Continue
    }

    fn turn(&mut self, unit_pos: UnitPos) -> Status {
        // don't handle this unit's turn if it has done it already
        if unit_pos.unit.rounds > self.rounds {
            return Status::Continue;
        }

        // identify all enemy units
        let enemy_type = unit_pos.unit.unit_type.enemy();
        let enemies = self.find_targets_of_type(enemy_type);

        if enemies.is_empty() {
            return Status::Win(unit_pos.unit.unit_type);
        }

        // current position
        let mut new_pos = unit_pos.pos.clone();

        // are we in range of an enemy? (eg. find enemies up/left/right/down)
        // if not move
        let enemies_in_range = self.enemies_in_range(&unit_pos);
        if enemies_in_range.is_empty() {
            let pos = self.mv(&unit_pos, &enemies);

            if unit_pos.pos != pos {
                self.move_unit(&unit_pos.pos, &pos);
                new_pos = pos;
            } else {
                // can't move, end turn
                // commit unit change
                let e = Elem::Unit(Unit {
                    unit_type: unit_pos.unit.unit_type,
                    hp: unit_pos.unit.hp,
                    rounds: unit_pos.unit.rounds + 1,
                });
                self.replace_elem(e, &unit_pos.pos);

                return Status::Continue;
            }
        }

        // get new element
        let unit_pos_new = UnitPos {
            unit: self.elem(new_pos.x, new_pos.y).unit().unwrap(),
            pos: new_pos,
        };

        // now, again, are we in range of an enemy?
        // if so, attack !!!
        let enemies_in_range = self.enemies_in_range(&unit_pos_new);
        if !enemies_in_range.is_empty() {
            self.attack(&enemies_in_range);
        }

        // commit unit change
        let e = Elem::Unit(Unit {
            unit_type: unit_pos_new.unit.unit_type,
            hp: unit_pos_new.unit.hp,
            rounds: unit_pos_new.unit.rounds + 1,
        });
        self.replace_elem(e, &unit_pos_new.pos);

        Status::Continue
    }

    fn attack(&mut self, enemies: &Vec<UnitPos>) {
        let min_hp = enemies.iter().map(|e| e.unit.hp).min().unwrap();

        for enemy in enemies.iter() {
            // prioritize enemies with low hp
            if enemy.unit.hp != min_hp {
                continue;
            }

            // attack power
            let atk_power = match enemy.unit.unit_type {
                UnitType::Elf => 3,
                UnitType::Goblin => self.elf_atk,
            } as isize;

            let hp_after_atk = enemy.unit.hp as isize - atk_power;
            if hp_after_atk <= 0 {
                self.delete_unit(&enemy.pos);
            } else {
                let e = Elem::Unit(Unit {
                    unit_type: enemy.unit.unit_type,
                    hp: hp_after_atk as usize,
                    rounds: enemy.unit.rounds,
                });
                self.replace_elem(e, &enemy.pos);
            }

            break;
        }
    }

    fn mv(&mut self, unit: &UnitPos, enemies: &Vec<UnitPos>) -> Pos {
        // find all open squares in range
        let open_squares = self.find_open_squares(&enemies);

        // if none available, just stop here
        if open_squares.is_empty() {
            return unit.pos.clone();
        }

        // find the way to get there, go Dijkstra
        let path = self.find_path(&unit.pos, &open_squares);

        path.unwrap_or(unit.pos.clone())
    }

    fn find_path(&self, start: &Pos, destinations: &Vec<Pos>) -> Option<Pos> {
        // create map copy for storing costs
        let mut costs = repeat(repeat(MAX).take(self.dims.0).collect::<Vec<_>>())
            .take(self.dims.1)
            .collect::<Vec<_>>();

        // create set from destination points
        let dest_set = destinations.iter().collect::<BTreeSet<_>>();

        // create queue of elements to process
        let mut to_process = VecDeque::new();

        // start with the source
        to_process.push_back(start.clone());
        costs[start.y][start.x] = 0;

        // find a cost to each open point in the map from the source
        let mut closest_dest: Option<Pos> = None;
        while !to_process.is_empty() {
            let pos = to_process.pop_front().unwrap();
            let cost = costs[pos.y][pos.x] + 1;

            // if we found one of our targets, use it
            if dest_set.contains(&pos) {
                let current_dest = pos.clone();

                if let Some(p) = closest_dest.clone() {
                    // keep the point that is the "smaller" in terms of order
                    if costs[current_dest.y][current_dest.x] == costs[p.y][p.x] && current_dest < p
                    {
                        closest_dest = Some(current_dest);
                    }
                } else {
                    closest_dest = Some(pos.clone());
                }
            }

            for p in pos.neighbours().iter() {
                // already processed
                if costs[p.y][p.x] != MAX {
                    continue;
                }

                if let Elem::Open = self.elem(p.x, p.y) {
                    costs[p.y][p.x] = cost;
                    to_process.push_back(p.clone());
                }
            }
        }

        // find closest destination
        if closest_dest.is_none() {
            return None;
        }

        // iterate back from destination to source to find the best starting points
        // neighbours are searched in reading order, so we prioritize top, then left
        let mut pos = closest_dest.unwrap().clone();
        while costs[pos.y][pos.x] != 1 {
            for p in pos.neighbours().iter() {
                if costs[p.y][p.x] < costs[pos.y][pos.x] {
                    pos = p.clone();
                }
            }
        }

        // we have the best source point!
        Some(pos)
    }

    fn find_open_squares(&self, enemies: &Vec<UnitPos>) -> Vec<Pos> {
        let mut squares = vec![];

        for enemy in enemies.iter() {
            // coords by reading order
            let coords = [
                (enemy.pos.x, enemy.pos.y - 1),
                (enemy.pos.x - 1, enemy.pos.y),
                (enemy.pos.x + 1, enemy.pos.y),
                (enemy.pos.x, enemy.pos.y + 1),
            ];

            for (x, y) in coords.iter() {
                if let Elem::Open = self.elem(*x, *y) {
                    squares.push(Pos { x: *x, y: *y });
                }
            }
        }

        squares
    }

    fn find_targets_of_type(&self, unit_type: UnitType) -> Vec<UnitPos> {
        let mut units = vec![];
        let (w, h) = self.dims;

        for y in 0..h {
            for x in 0..w {
                let elem = self.elem(x, y);
                if let Some(unit) = elem.unit() {
                    if unit.unit_type == unit_type {
                        units.push(UnitPos {
                            unit,
                            pos: Pos { x, y },
                        });
                    }
                }
            }
        }

        units
    }

    fn enemies_in_range(&self, unit_pos: &UnitPos) -> Vec<UnitPos> {
        let mut units = vec![];

        // neighbours by reading order
        let neighbours = unit_pos.pos.neighbours();

        for pos in neighbours.iter() {
            let e = self.elem(pos.x, pos.y).unit();
            if let Some(u) = e {
                if u.unit_type == unit_pos.unit.unit_type.enemy() {
                    units.push(UnitPos {
                        unit: u,
                        pos: Pos { x: pos.x, y: pos.y },
                    });
                }
            }
        }

        units
    }

    fn elem(&self, x: usize, y: usize) -> Elem {
        self.cells[y][x]
    }

    fn replace_elem(&mut self, elem: Elem, pos: &Pos) {
        self.cells[pos.y][pos.x] = elem;
    }

    fn delete_unit(&mut self, pos: &Pos) {
        self.replace_elem(Elem::Open, pos);
    }

    fn move_unit(&mut self, from: &Pos, to: &Pos) {
        let elem = self.elem(from.x, from.y);
        self.delete_unit(&from);
        self.replace_elem(elem, to);
    }
}

#[derive(Eq, PartialEq)]
enum Status {
    Continue,
    Win(UnitType),
}

#[derive(Copy, Clone)]
pub enum Elem {
    Unit(Unit),
    Open,
    Wall,
}

impl Elem {
    fn from_char(c: char) -> Option<Elem> {
        match c {
            '.' => Some(Elem::Open),
            'E' => Some(Elem::Unit(Unit::new(UnitType::Elf))),
            'G' => Some(Elem::Unit(Unit::new(UnitType::Goblin))),
            '#' => Some(Elem::Wall),
            _ => None,
        }
    }

    fn unit(&self) -> Option<Unit> {
        match self {
            Elem::Unit(u) => Some(u.clone()),
            _ => None,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Unit {
    unit_type: UnitType,
    hp: usize,
    rounds: usize,
}

impl Unit {
    fn new(t: UnitType) -> Self {
        Self {
            unit_type: t,
            hp: 200,
            rounds: 0,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UnitType {
    Elf,
    Goblin,
}

impl UnitType {
    fn enemy(&self) -> UnitType {
        match self {
            UnitType::Elf => UnitType::Goblin,
            UnitType::Goblin => UnitType::Elf,
        }
    }
}

struct UnitPos {
    unit: Unit,
    pos: Pos,
}

#[derive(Clone, Debug, Eq)]
struct Pos {
    x: usize,
    y: usize,
}

impl Pos {
    fn neighbours(&self) -> [Pos; 4] {
        [
            Pos {
                x: self.x,
                y: self.y - 1,
            },
            Pos {
                x: self.x - 1,
                y: self.y,
            },
            Pos {
                x: self.x + 1,
                y: self.y,
            },
            Pos {
                x: self.x,
                y: self.y + 1,
            },
        ]
    }
}

impl Ord for Pos {
    fn cmp(&self, other: &Self) -> Ordering {
        let a = (self.y, self.x);
        let b = (other.y, other.x);
        a.cmp(&b)
    }
}

impl PartialOrd for Pos {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Pos {
    fn eq(&self, other: &Self) -> bool {
        let a = (self.y, self.x);
        let b = (other.y, other.x);
        a == b
    }
}
