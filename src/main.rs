extern crate chrono;
extern crate regex;

mod solver;
mod solutions;

use solutions::*;
use solver::Solver;
use std::env;

fn main() {
    let day = env::args()
        .nth(1)
        .unwrap_or(String::from("1"));

    match day.as_str() {
        "1" => Day01::solve(),
        "2" => Day02::solve(),
        "3" => Day03::solve(),
        "4" => Day04::solve(),
        "5" => Day05::solve(),
        "6" => Day06::solve(),
        "7" => Day07::solve(),
        d => println!("Day {} hasn't been solved yet :(", d)
    }
}
