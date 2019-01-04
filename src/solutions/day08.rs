use crate::solver::Solver;
use std::io;
use std::io::BufRead;
use std::io::BufReader;

pub struct Day08;

impl Solver for Day08 {
    type Input = Vec<u32>;
    type Output1 = u32;
    type Output2 = u32;

    fn day() -> u32 {
        8
    }

    fn parse_input<R: io::Read>(r: R) -> Vec<u32> {
        let r = BufReader::new(r);
        r.split(b' ')
            .filter_map(|s| s.ok())
            .filter_map(|s| String::from_utf8(s).ok())
            .filter_map(|s| s.parse().ok())
            .collect()
    }

    fn solve_first(input: &Vec<u32>) -> u32 {
        let mut it = input.iter();
        let tree = parse_nodes(&mut it);

        tree.unwrap().meta_sum
    }

    fn solve_second(input: &Vec<u32>) -> u32 {
        let mut it = input.iter();
        let tree = parse_nodes(&mut it);

        tree.unwrap().value
    }
}

struct Node {
    children: Vec<Node>,
    metadata: Vec<u32>,
    meta_sum: u32,
    value: u32,
}

fn parse_nodes<'a>(it: &mut impl Iterator<Item = &'a u32>) -> Option<Node> {
    let num_nodes = it.next()?;
    let num_meta = it.next()?;

    let mut node = Node {
        children: vec![],
        metadata: vec![],
        meta_sum: 0,
        value: 0,
    };

    for _ in 0..*num_nodes {
        if let Some(child_node) = parse_nodes(it) {
            node.meta_sum += child_node.meta_sum;
            node.children.push(child_node);
        }
    }

    for _ in 0..*num_meta {
        let meta = it.next()?;
        node.meta_sum += *meta;
        node.metadata.push(*meta);
    }

    // value
    if node.children.is_empty() {
        node.value = node.metadata.iter().sum();
    } else {
        for i in node.metadata.iter() {
            if *i == 0 {
                continue;
            }

            let idx = *i as usize - 1;
            if let Some(child) = node.children.get(idx) {
                node.value += child.value;
            }
        }
    }

    Some(node)
}
