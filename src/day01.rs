use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;
use std::collections::HashSet;

fn lines<P: AsRef<Path>>(p: P) -> io::Result<Vec<isize>> {
    let f = File::open(p)?;
    let file = BufReader::new(&f);
    let v = file.lines()
        .filter_map(|l| l.ok())
        .map(|l| l.parse::<isize>())
        .filter_map(|i| i.ok())
        .collect();
    Ok(v)
}

fn find_first_repeat(values: &Vec<isize>) -> isize {
    let mut frequencies = HashSet::new();
    let mut frequency = 0;

    for v in values.iter().cycle() {
        frequency += v;
        if frequencies.contains(&frequency) {
            return frequency;
        }
        else {
            frequencies.insert(frequency);
        }
    }

    unreachable!()
}

pub fn solve() {
    let values = lines("input/day01").expect("unable to open input file");

    // first part
    let sum: isize = values.iter().sum();
    println!("total: {}", sum);

    // second part
    let first_repeat = find_first_repeat(&values);
    println!("first_repeat: {}", first_repeat);
}
