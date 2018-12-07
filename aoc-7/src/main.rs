use nom::types::CompleteStr;
use nom::*;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Read;

#[derive(Clone, Copy, Debug)]
struct Edge {
    start: char,
    end: char,
}

named!(edge <CompleteStr, Edge>, do_parse!(
    tag!("Step ") >>
    start: map!(take!(1), |s| s.chars().next().unwrap()) >>
    tag!(" must be finished before step ") >>
    end: map!(take!(1), |s| s.chars().next().unwrap()) >>
    tag!(" can begin.") >>
    line_ending >>
    (Edge { start, end })
));

named!(parse <CompleteStr, Vec<Edge>>, many1!(edge));

#[derive(Clone, Debug)]
struct Dependencies {
    deps: HashMap<char, HashSet<char>>,
}

impl Dependencies {
    fn new() -> Self {
        Self {
            deps: HashMap::new(),
        }
    }

    fn insert(&mut self, edge: Edge) {
        self.deps
            .entry(edge.end)
            .or_insert_with(HashSet::new)
            .insert(edge.start);
        if !self.deps.contains_key(&edge.start) {
            self.deps.insert(edge.start, HashSet::new());
        }
    }

    fn satisfy_dep(&mut self, dep: char) {
        for deps in self.deps.values_mut() {
            deps.remove(&dep);
        }
    }

    fn next_step(&mut self) -> char {
        let mut candidates: Vec<_> = self
            .deps
            .iter()
            .filter(|&(_, deps)| deps.is_empty())
            .map(|(k, _)| *k)
            .collect();
        candidates.sort();
        self.deps.remove(&candidates[0]);
        candidates[0]
    }

    fn is_empty(&self) -> bool {
        self.deps.is_empty()
    }
}

fn main() {
    let mut file = File::open("input").unwrap();
    let mut input = String::new();
    let _ = file.read_to_string(&mut input);

    let edges = parse(CompleteStr(&input)).unwrap().1;

    let mut dependencies = Dependencies::new();
    for edge in edges {
        dependencies.insert(edge);
    }

    print!("Part 1: ");

    while !dependencies.is_empty() {
        let next_step = dependencies.next_step();
        print!("{}", next_step);
        dependencies.satisfy_dep(next_step);
    }
    println!();
}
