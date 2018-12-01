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
        d => println!("Day {} hasn't been solved yet :(", d)
    }
}
