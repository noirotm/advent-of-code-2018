extern crate chrono;
extern crate regex;
extern crate core;

mod solutions;
mod solver;

use solutions::*;
use solver::Solver;
use std::env;

fn main() {
    let day = env::args().nth(1).unwrap_or(String::from("1"));

    match day.as_str() {
        "1" => Day01::solve(),
        "2" => Day02::solve(),
        "3" => Day03::solve(),
        "4" => Day04::solve(),
        "5" => Day05::solve(),
        "6" => Day06::solve(),
        "7" => Day07::solve(),
        "8" => Day08::solve(),
        "9" => Day09::solve(),
        "10" => Day10::solve(),
        "11" => Day11::solve(),
        "12" => Day12::solve(),
        "13" => Day13::solve(),
        "14" => Day14::solve(),
        "15" => Day15::solve(),
        "16" => Day16::solve(),
        "17" => Day17::solve(),
        d => println!("Day {} hasn't been solved yet :(", d),
    }
}
