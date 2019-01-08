mod solutions;
mod solver;

use crate::solutions::exec_day;
use std::env;

fn main() {
    let day = env::args().nth(1).unwrap_or(String::from("1"));
    exec_day(day.as_str());
}
