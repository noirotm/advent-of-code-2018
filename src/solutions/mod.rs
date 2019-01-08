use crate::solver::Solver;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;

pub fn exec_day(day: &str) {
    match day {
        "1" => day01::Day01{}.solve(day),
        "2" => day02::Day02{}.solve(day),
        "3" => day03::Day03{}.solve(day),
        "4" => day04::Day04{}.solve(day),
        "5" => day05::Day05{}.solve(day),
        "6" => day06::Day06{}.solve(day),
        "7" => day07::Day07{}.solve(day),
        "8" => day08::Day08{}.solve(day),
        "9" => day09::Day09{}.solve(day),
        "10" => day10::Day10{}.solve(day),
        "11" => day11::Day11{}.solve(day),
        "12" => day12::Day12{}.solve(day),
        "13" => day13::Day13{}.solve(day),
        "14" => day14::Day14{}.solve(day),
        "15" => day15::Day15{}.solve(day),
        "16" => day16::Day16{}.solve(day),
        "17" => day17::Day17{}.solve(day),
        "18" => day18::Day18{}.solve(day),
        "19" => day19::Day19{}.solve(day),
        "20" => day20::Day20{}.solve(day),
        "21" => day21::Day21{}.solve(day),
        d => println!("Day {} hasn't been solved yet :(", d),
    }
}
