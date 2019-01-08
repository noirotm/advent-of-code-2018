use crate::solver::Solver;

mod day01;
pub use self::day01::Day01;

mod day02;
pub use self::day02::Day02;

mod day03;
pub use self::day03::Day03;

mod day04;
pub use self::day04::Day04;

mod day05;
pub use self::day05::Day05;

mod day06;
pub use self::day06::Day06;

mod day07;
pub use self::day07::Day07;

mod day08;
pub use self::day08::Day08;

mod day09;
pub use self::day09::Day09;

mod day10;
pub use self::day10::Day10;

mod day11;
pub use self::day11::Day11;

mod day12;
pub use self::day12::Day12;

mod day13;
pub use self::day13::Day13;

mod day14;
pub use self::day14::Day14;

mod day15;
pub use self::day15::Day15;

mod day16;
pub use self::day16::Day16;

mod day17;
pub use self::day17::Day17;

mod day18;
pub use self::day18::Day18;

mod day19;
pub use self::day19::Day19;

mod day20;
pub use self::day20::Day20;

mod day21;
pub use self::day21::Day21;

pub fn exec_day(day: &str) {
    match day {
        "1" => Day01{}.solve(day),
        "2" => Day02{}.solve(day),
        "3" => Day03{}.solve(day),
        "4" => Day04{}.solve(day),
        "5" => Day05{}.solve(day),
        "6" => Day06{}.solve(day),
        "7" => Day07{}.solve(day),
        "8" => Day08{}.solve(day),
        "9" => Day09{}.solve(day),
        "10" => Day10{}.solve(day),
        "11" => Day11{}.solve(day),
        "12" => Day12{}.solve(day),
        "13" => Day13{}.solve(day),
        "14" => Day14{}.solve(day),
        "15" => Day15{}.solve(day),
        "16" => Day16{}.solve(day),
        "17" => Day17{}.solve(day),
        "18" => Day18{}.solve(day),
        "19" => Day19{}.solve(day),
        "20" => Day20{}.solve(day),
        "21" => Day21{}.solve(day),
        d => println!("Day {} hasn't been solved yet :(", d),
    }
}
