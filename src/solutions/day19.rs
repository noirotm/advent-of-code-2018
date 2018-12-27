use regex::Regex;
use solver::Solver;
use std::io;
use std::io::BufRead;
use std::io::BufReader;

pub struct Day19;

impl Solver for Day19 {
    type Input = Program;
    type Output1 = u64;
    type Output2 = u64;

    fn day() -> u32 {
        19
    }

    fn parse_input<R: io::Read>(r: R) -> Program {
        let mut lines = BufReader::new(r).lines();

        // parse first line
        let ip_register = lines
            .next()
            .and_then(|s| s.ok())
            .and_then(|s| s.chars().nth(4))
            .and_then(|c| c.to_digit(10))
            .unwrap_or(0) as usize;

        // parse instructions
        let inst_re = Regex::new(r"(\w+) (\d+) (\d+) (\d+)").unwrap();
        let instructions = lines
            .filter_map(|l| l.ok())
            .filter_map(|s| {
                inst_re.captures(s.as_str()).and_then(|c| {
                    Some(Inst {
                        opcode: Opcode::parse(c.get(1)?.as_str())?,
                        args: [
                            c.get(2)?.as_str().parse().ok()?,
                            c.get(3)?.as_str().parse().ok()?,
                            c.get(4)?.as_str().parse().ok()?,
                        ],
                    })
                })
            })
            .collect();

        Program {
            ip_register,
            instructions,
        }
    }

    fn solve_first(input: &Program) -> u64 {
        let mut vm = Machine::new();
        input.execute(&mut vm);

        vm.registers[0]
    }

    fn solve_second(input: &Program) -> u64 {
        let mut vm = Machine::new();
        vm.registers[0] = 1;

        input.execute_with_ip(&mut vm, 0);

        vm.registers[0]
    }
}

fn quick_solve(vm: &mut Machine) {
    /*vm.registers[2] = vm.registers[3] * vm.registers[1]; // 3
    if vm.registers[2] == vm.registers[5] { // 4
        vm.registers[2] = 1;
    } else {
        vm.registers[2] = 0;
    }
    // if r2 == r5 skip #6 (goto #7)
    vm.registers[4] = vm.registers[2] + vm.registers[4]; // 5

    // else skip #7 (goto #8)
    vm.registers[4] = vm.registers[4] + 1; // 6

    vm.registers[0] = vm.registers[3] + vm.registers[0]; // 7

    vm.registers[1] = vm.registers[1] + 1; // 8

    if vm.registers[1] > vm.registers[5] { // 9
        vm.registers[2] = 1;
    } else {
        vm.registers[2] = 0;
    }

    // if r1 > r5 skip #11 (goto #12)
    vm.registers[4] = vm.registers[4] + vm.registers[2]; // 10

    // goto #3
    vm.registers[4] = 2; // 11 */

    /*loop {
        vm.registers[2] = vm.registers[3] * vm.registers[1];
        if vm.registers[2] == vm.registers[5] {
            vm.registers[0] = vm.registers[3] + vm.registers[0];
        }
        vm.registers[1] = vm.registers[1] + 1;

        if vm.registers[1] > vm.registers[5] {
            break;
        }
    }*/

    if vm.registers[5] % vm.registers[3] == 0 {
        vm.registers[0] = vm.registers[3] + vm.registers[0];
    }
    vm.registers[1] = vm.registers[5] + 1;
    vm.registers[2] = 1;
    vm.registers[4] = 11;
}

#[derive(Debug)]
pub enum Opcode {
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

impl Opcode {
    fn parse(s: &str) -> Option<Self> {
        match s {
            "addr" => Some(Opcode::Addr),
            "addi" => Some(Opcode::Addi),
            "mulr" => Some(Opcode::Mulr),
            "muli" => Some(Opcode::Muli),
            "banr" => Some(Opcode::Banr),
            "bani" => Some(Opcode::Bani),
            "borr" => Some(Opcode::Borr),
            "bori" => Some(Opcode::Bori),
            "setr" => Some(Opcode::Setr),
            "seti" => Some(Opcode::Seti),
            "gtir" => Some(Opcode::Gtir),
            "gtri" => Some(Opcode::Gtri),
            "gtrr" => Some(Opcode::Gtrr),
            "eqir" => Some(Opcode::Eqir),
            "eqri" => Some(Opcode::Eqri),
            "eqrr" => Some(Opcode::Eqrr),
            _ => None,
        }
    }
}

pub struct Inst {
    opcode: Opcode,
    args: [u8; 3],
}

pub struct Program {
    ip_register: usize,
    instructions: Vec<Inst>,
}

impl Program {
    fn execute(&self, vm: &mut Machine) {
        let mut ip = 0;
        loop {
            // fetch instruction
            if let Some(inst) = self.instructions.get(ip as usize) {
                // prepare ip register
                vm.registers[self.ip_register] = ip as u64;

                /*print!(
                    "ip={} {:?} {:?} {} {} {} ",
                    ip, vm.registers, inst.opcode, inst.args[0], inst.args[1], inst.args[2]
                );*/

                // exec
                vm.exec(inst);

                //println!("{:?}", vm.registers);

                // restore ip
                ip = vm.registers[self.ip_register] as usize;

                // increment for next instruction
                ip += 1;

            //let _ = io::stdin().read(&mut [0u8]).unwrap();
            } else {
                break;
            }
        }
    }

    fn execute_with_ip(&self, vm: &mut Machine, ip: usize) {
        let mut ip = ip;
        loop {
            // fetch instruction
            if let Some(inst) = self.instructions.get(ip as usize) {
                // prepare ip register
                vm.registers[self.ip_register] = ip as u64;

                /*print!(
                    "ip={} {:?} {:?} {} {} {} ",
                    ip, vm.registers, inst.opcode, inst.args[0], inst.args[1], inst.args[2]
                );*/

                // exec
                if ip == 3 {
                    quick_solve(vm);
                } else {
                    vm.exec(inst);
                }

                //println!("{:?}", vm.registers);

                // restore ip
                ip = vm.registers[self.ip_register] as usize;

                // increment for next instruction
                ip += 1;

            //let _ = io::stdin().read(&mut [0u8]).unwrap();
            } else {
                break;
            }
        }
    }
}

struct Machine {
    registers: [u64; 6],
}

impl Machine {
    fn new() -> Self {
        Self { registers: [0; 6] }
    }

    fn exec(&mut self, inst: &Inst) {
        match inst.opcode {
            Opcode::Addr => self.addr(inst.args[0], inst.args[1], inst.args[2]),
            Opcode::Addi => self.addi(inst.args[0], inst.args[1], inst.args[2]),
            Opcode::Mulr => self.mulr(inst.args[0], inst.args[1], inst.args[2]),
            Opcode::Muli => self.muli(inst.args[0], inst.args[1], inst.args[2]),
            Opcode::Banr => self.banr(inst.args[0], inst.args[1], inst.args[2]),
            Opcode::Bani => self.bani(inst.args[0], inst.args[1], inst.args[2]),
            Opcode::Borr => self.borr(inst.args[0], inst.args[1], inst.args[2]),
            Opcode::Bori => self.bori(inst.args[0], inst.args[1], inst.args[2]),
            Opcode::Setr => self.setr(inst.args[0], inst.args[1], inst.args[2]),
            Opcode::Seti => self.seti(inst.args[0], inst.args[1], inst.args[2]),
            Opcode::Gtir => self.gtir(inst.args[0], inst.args[1], inst.args[2]),
            Opcode::Gtri => self.gtri(inst.args[0], inst.args[1], inst.args[2]),
            Opcode::Gtrr => self.gtrr(inst.args[0], inst.args[1], inst.args[2]),
            Opcode::Eqir => self.eqir(inst.args[0], inst.args[1], inst.args[2]),
            Opcode::Eqri => self.eqri(inst.args[0], inst.args[1], inst.args[2]),
            Opcode::Eqrr => self.eqrr(inst.args[0], inst.args[1], inst.args[2]),
        }
    }

    fn addr(&mut self, r1: u8, r2: u8, r3: u8) {
        let val = self.reg(r1) + self.reg(r2);
        self.set_reg(r3, val);
    }

    fn addi(&mut self, r1: u8, v2: u8, r3: u8) {
        let val = self.reg(r1) + v2 as u64;
        self.set_reg(r3, val);
    }

    fn mulr(&mut self, r1: u8, r2: u8, r3: u8) {
        let val = self.reg(r1) * self.reg(r2);
        self.set_reg(r3, val);
    }

    fn muli(&mut self, r1: u8, v2: u8, r3: u8) {
        let val = self.reg(r1) * v2 as u64;
        self.set_reg(r3, val);
    }

    fn banr(&mut self, r1: u8, r2: u8, r3: u8) {
        let val = self.reg(r1) & self.reg(r2);
        self.set_reg(r3, val);
    }

    fn bani(&mut self, r1: u8, v2: u8, r3: u8) {
        let val = self.reg(r1) & v2 as u64;
        self.set_reg(r3, val);
    }

    fn borr(&mut self, r1: u8, r2: u8, r3: u8) {
        let val = self.reg(r1) | self.reg(r2);
        self.set_reg(r3, val);
    }

    fn bori(&mut self, r1: u8, v2: u8, r3: u8) {
        let val = self.reg(r1) | v2 as u64;
        self.set_reg(r3, val);
    }

    fn setr(&mut self, r1: u8, _: u8, r3: u8) {
        let val = self.reg(r1);
        self.set_reg(r3, val);
    }

    fn seti(&mut self, v1: u8, _: u8, r3: u8) {
        self.set_reg(r3, v1 as u64);
    }

    fn gtir(&mut self, v1: u8, r2: u8, r3: u8) {
        let val = if v1 as u64 > self.reg(r2) { 1 } else { 0 };
        self.set_reg(r3, val);
    }

    fn gtri(&mut self, r1: u8, v2: u8, r3: u8) {
        let val = if self.reg(r1) > v2 as u64 { 1 } else { 0 };
        self.set_reg(r3, val);
    }

    fn gtrr(&mut self, r1: u8, r2: u8, r3: u8) {
        let val = if self.reg(r1) > self.reg(r2) { 1 } else { 0 };
        self.set_reg(r3, val);
    }

    fn eqir(&mut self, v1: u8, r2: u8, r3: u8) {
        let val = if v1 as u64 == self.reg(r2) { 1 } else { 0 };
        self.set_reg(r3, val);
    }

    fn eqri(&mut self, r1: u8, v2: u8, r3: u8) {
        let val = if self.reg(r1) == v2 as u64 { 1 } else { 0 };
        self.set_reg(r3, val);
    }

    fn eqrr(&mut self, r1: u8, r2: u8, r3: u8) {
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
