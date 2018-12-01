use std::fs::File;
use std::io;
use std::path::Path;
use std::fmt::Display;

fn input_file(day: u32) -> String {
    format!("input/day{:02}", day)
}

pub trait Solver {
    type Input;
    type Output1: Display;
    type Output2: Display;

    fn day() -> u32;
    fn parse_input<R: io::Read>(r: R) -> io::Result<Self::Input>;
    fn solve_first(input: &Self::Input) -> Self::Output1;
    fn solve_second(input: &Self::Input) -> Self::Output2;

    fn load_input<P: AsRef<Path>>(p: P) -> io::Result<Self::Input> {
        let f = File::open(p)?;
        Self::parse_input(f)
    }

    fn solve() {
        let input =
            Self::load_input(input_file(Self::day())).expect("unable to open input file");
        let s1 = Self::solve_first(&input);
        let s2 = Self::solve_second(&input);
        println!("Solution 1: {}", s1);
        println!("Solution 2: {}", s2);
    }
}
