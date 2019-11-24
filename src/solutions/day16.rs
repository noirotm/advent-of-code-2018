use crate::solver::Solver;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io;
use std::io::BufRead;
use std::io::BufReader;

pub struct Problem;

impl Solver for Problem {
    type Input = Input;
    type Output1 = usize;
    type Output2 = u64;

    fn parse_input<R: io::Read>(&self, r: R) -> Input {
        let mut lines = BufReader::new(r).lines().filter_map(|l| l.ok());

        let reg_re = Regex::new(r"(\d), (\d), (\d), (\d)").unwrap();
        let opc_re = Regex::new(r"(\d+) (\d+) (\d+) (\d+)").unwrap();

        let mut tests = vec![];

        loop {
            let before = lines.next().unwrap();
            let c = reg_re.captures(before.as_str());
            let before = if let Some(c) = c {
                (
                    c.get(1).unwrap().as_str().parse().unwrap(),
                    c.get(2).unwrap().as_str().parse().unwrap(),
                    c.get(3).unwrap().as_str().parse().unwrap(),
                    c.get(4).unwrap().as_str().parse().unwrap(),
                )
            } else {
                break;
            };

            let opcode = lines.next().unwrap();
            let c = opc_re.captures(opcode.as_str());
            let opcode = if let Some(c) = c {
                (
                    c.get(1).unwrap().as_str().parse().unwrap(),
                    c.get(2).unwrap().as_str().parse().unwrap(),
                    c.get(3).unwrap().as_str().parse().unwrap(),
                    c.get(4).unwrap().as_str().parse().unwrap(),
                )
            } else {
                break;
            };

            let after = lines.next().unwrap();
            let c = reg_re.captures(after.as_str());
            let after = if let Some(c) = c {
                (
                    c.get(1).unwrap().as_str().parse().unwrap(),
                    c.get(2).unwrap().as_str().parse().unwrap(),
                    c.get(3).unwrap().as_str().parse().unwrap(),
                    c.get(4).unwrap().as_str().parse().unwrap(),
                )
            } else {
                break;
            };

            tests.push(TestCase {
                before,
                after,
                opcode,
            });

            lines.next();
        }

        let program = lines
            .filter_map(|s| {
                opc_re.captures(s.as_str()).and_then(|c| {
                    Some((
                        c.get(1)?.as_str().parse().ok()?,
                        c.get(2)?.as_str().parse().ok()?,
                        c.get(3)?.as_str().parse().ok()?,
                        c.get(4)?.as_str().parse().ok()?,
                    ))
                })
            })
            .collect();

        Input { tests, program }
    }

    fn solve_first(&self, input: &Input) -> usize {
        let mut count = 0;

        for tc in input.tests.iter() {
            let results = test_all_for_input(tc.before, tc.opcode);

            let n = results.iter().filter(|&&(_, r)| r == tc.after).count();

            if n >= 3 {
                count += 1;
            }
        }

        count
    }

    fn solve_second(&self, input: &Input) -> u64 {
        let mappings = find_opcode_mapping(&input.tests);
        println!("{:#?}", mappings);
        println!("{:#?}", input.program);

        let mut m = Machine::load((0, 0, 0, 0));
        for opcode in input.program.iter() {
            let instr = mappings.get(&opcode.0).unwrap();
            m.exec((*instr, opcode.1, opcode.2, opcode.3));
        }

        m.registers[0]
    }
}

fn find_opcode_mapping(tests: &Vec<TestCase>) -> HashMap<u8, Inst> {
    let mut possible_matches = vec![HashSet::<Inst>::new(); 16];

    for tc in tests.iter() {
        let results = test_all_for_input(tc.before, tc.opcode);
        let matches = results
            .iter()
            .filter(|&&(_, r)| r == tc.after)
            .map(|(i, _)| i.clone())
            .collect::<HashSet<_>>();

        //println!("{} => {:#?}", tc.opcode.0, matches);

        let existing_matches = possible_matches[tc.opcode.0 as usize]
            .iter()
            .cloned()
            .collect::<HashSet<_>>();
        if existing_matches.is_empty() {
            possible_matches[tc.opcode.0 as usize] = matches;
        } else {
            let inter = matches
                .intersection(&existing_matches)
                .cloned()
                .collect::<HashSet<_>>();
            possible_matches[tc.opcode.0 as usize] = inter;
        }
    }

    //println!("{:#?}", possible_matches);

    // final map
    let mut mappings: HashMap<u8, Inst> = HashMap::new();

    // remove in ascending order
    loop {
        let (instr, opcode) = {
            let (instr, opcodes) = possible_matches
                .iter()
                .enumerate()
                .find(|(_, m)| m.len() == 1)
                .expect("unsolvable");

            (instr as u8, opcodes.iter().next().unwrap().clone())
        };

        mappings.insert(instr, opcode);

        for m in possible_matches.iter_mut() {
            m.remove(&opcode);
        }

        if mappings.len() == 16 {
            break;
        }
    }

    mappings
}

pub struct Input {
    tests: Vec<TestCase>,
    program: Vec<(u8, u8, u8, u8)>,
}

#[derive(Debug)]
pub struct TestCase {
    before: (u64, u64, u64, u64),
    after: (u64, u64, u64, u64),
    opcode: (u8, u8, u8, u8),
}

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
enum Inst {
    Addr,
    Addi,
    Mulr,
    Muli,
    Banr,
    Bani,
    Borr,
    Bori,
    Setr,
    Seti,
    Gtir,
    Gtri,
    Gtrr,
    Eqir,
    Eqri,
    Eqrr,
}

struct Machine {
    registers: [u64; 4],
}

impl Machine {
    fn load(values: (u64, u64, u64, u64)) -> Self {
        Machine {
            registers: [values.0, values.1, values.2, values.3],
        }
    }

    fn values(&self) -> (u64, u64, u64, u64) {
        (
            self.registers[0],
            self.registers[1],
            self.registers[2],
            self.registers[3],
        )
    }

    fn exec(&mut self, (r0, r1, r2, r3): (Inst, u8, u8, u8)) {
        match r0 {
            Inst::Addr => self.addr((0, r1, r2, r3)),
            Inst::Addi => self.addi((0, r1, r2, r3)),
            Inst::Mulr => self.mulr((0, r1, r2, r3)),
            Inst::Muli => self.muli((0, r1, r2, r3)),
            Inst::Banr => self.banr((0, r1, r2, r3)),
            Inst::Bani => self.bani((0, r1, r2, r3)),
            Inst::Borr => self.borr((0, r1, r2, r3)),
            Inst::Bori => self.bori((0, r1, r2, r3)),
            Inst::Setr => self.setr((0, r1, r2, r3)),
            Inst::Seti => self.seti((0, r1, r2, r3)),
            Inst::Gtir => self.gtir((0, r1, r2, r3)),
            Inst::Gtri => self.gtri((0, r1, r2, r3)),
            Inst::Gtrr => self.gtrr((0, r1, r2, r3)),
            Inst::Eqir => self.eqir((0, r1, r2, r3)),
            Inst::Eqri => self.eqri((0, r1, r2, r3)),
            Inst::Eqrr => self.eqrr((0, r1, r2, r3)),
        }
    }

    fn addr(&mut self, (_, r1, r2, r3): (u8, u8, u8, u8)) {
        let val = self.reg(r1) + self.reg(r2);
        self.set_reg(r3, val);
    }

    fn addi(&mut self, (_, r1, v2, r3): (u8, u8, u8, u8)) {
        let val = self.reg(r1) + v2 as u64;
        self.set_reg(r3, val);
    }

    fn mulr(&mut self, (_, r1, r2, r3): (u8, u8, u8, u8)) {
        let val = self.reg(r1) * self.reg(r2);
        self.set_reg(r3, val);
    }

    fn muli(&mut self, (_, r1, v2, r3): (u8, u8, u8, u8)) {
        let val = self.reg(r1) * v2 as u64;
        self.set_reg(r3, val);
    }

    fn banr(&mut self, (_, r1, r2, r3): (u8, u8, u8, u8)) {
        let val = self.reg(r1) & self.reg(r2);
        self.set_reg(r3, val);
    }

    fn bani(&mut self, (_, r1, v2, r3): (u8, u8, u8, u8)) {
        let val = self.reg(r1) & v2 as u64;
        self.set_reg(r3, val);
    }

    fn borr(&mut self, (_, r1, r2, r3): (u8, u8, u8, u8)) {
        let val = self.reg(r1) | self.reg(r2);
        self.set_reg(r3, val);
    }

    fn bori(&mut self, (_, r1, v2, r3): (u8, u8, u8, u8)) {
        let val = self.reg(r1) | v2 as u64;
        self.set_reg(r3, val);
    }

    fn setr(&mut self, (_, r1, _, r3): (u8, u8, u8, u8)) {
        let val = self.reg(r1);
        self.set_reg(r3, val);
    }

    fn seti(&mut self, (_, v1, _, r3): (u8, u8, u8, u8)) {
        self.set_reg(r3, v1 as u64);
    }

    fn gtir(&mut self, (_, v1, r2, r3): (u8, u8, u8, u8)) {
        let val = if v1 as u64 > self.reg(r2) { 1 } else { 0 };
        self.set_reg(r3, val);
    }

    fn gtri(&mut self, (_, r1, v2, r3): (u8, u8, u8, u8)) {
        let val = if self.reg(r1) > v2 as u64 { 1 } else { 0 };
        self.set_reg(r3, val);
    }

    fn gtrr(&mut self, (_, r1, r2, r3): (u8, u8, u8, u8)) {
        let val = if self.reg(r1) > self.reg(r2) { 1 } else { 0 };
        self.set_reg(r3, val);
    }

    fn eqir(&mut self, (_, v1, r2, r3): (u8, u8, u8, u8)) {
        let val = if v1 as u64 == self.reg(r2) { 1 } else { 0 };
        self.set_reg(r3, val);
    }

    fn eqri(&mut self, (_, r1, v2, r3): (u8, u8, u8, u8)) {
        let val = if self.reg(r1) == v2 as u64 { 1 } else { 0 };
        self.set_reg(r3, val);
    }

    fn eqrr(&mut self, (_, r1, r2, r3): (u8, u8, u8, u8)) {
        let val = if self.reg(r1) == self.reg(r2) { 1 } else { 0 };
        self.set_reg(r3, val);
    }

    fn reg(&self, n: u8) -> u64 {
        self.registers[n as usize]
    }

    fn set_reg(&mut self, n: u8, val: u64) {
        self.registers[n as usize] = val;
    }
}

fn test_all_for_input(
    input: (u64, u64, u64, u64),
    opcode: (u8, u8, u8, u8),
) -> Vec<(Inst, (u64, u64, u64, u64))> {
    let mut output = vec![];

    let mut cpu = Machine::load(input);
    cpu.addr(opcode);
    output.push((Inst::Addr, cpu.values()));

    cpu = Machine::load(input);
    cpu.addi(opcode);
    output.push((Inst::Addi, cpu.values()));

    cpu = Machine::load(input);
    cpu.mulr(opcode);
    output.push((Inst::Mulr, cpu.values()));

    cpu = Machine::load(input);
    cpu.muli(opcode);
    output.push((Inst::Muli, cpu.values()));

    cpu = Machine::load(input);
    cpu.banr(opcode);
    output.push((Inst::Banr, cpu.values()));

    cpu = Machine::load(input);
    cpu.bani(opcode);
    output.push((Inst::Bani, cpu.values()));

    cpu = Machine::load(input);
    cpu.borr(opcode);
    output.push((Inst::Borr, cpu.values()));

    cpu = Machine::load(input);
    cpu.bori(opcode);
    output.push((Inst::Bori, cpu.values()));

    cpu = Machine::load(input);
    cpu.setr(opcode);
    output.push((Inst::Setr, cpu.values()));

    cpu = Machine::load(input);
    cpu.seti(opcode);
    output.push((Inst::Seti, cpu.values()));

    cpu = Machine::load(input);
    cpu.gtir(opcode);
    output.push((Inst::Gtir, cpu.values()));

    cpu = Machine::load(input);
    cpu.gtri(opcode);
    output.push((Inst::Gtri, cpu.values()));

    cpu = Machine::load(input);
    cpu.gtrr(opcode);
    output.push((Inst::Gtrr, cpu.values()));

    cpu = Machine::load(input);
    cpu.eqir(opcode);
    output.push((Inst::Eqir, cpu.values()));

    cpu = Machine::load(input);
    cpu.eqri(opcode);
    output.push((Inst::Eqri, cpu.values()));

    cpu = Machine::load(input);
    cpu.eqrr(opcode);
    output.push((Inst::Eqrr, cpu.values()));

    output
}
