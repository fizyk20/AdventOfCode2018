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

    fn next_step(&mut self) -> Option<char> {
        let mut candidates: Vec<_> = self
            .deps
            .iter()
            .filter(|&(_, deps)| deps.is_empty())
            .map(|(k, _)| *k)
            .collect();
        if !candidates.is_empty() {
            candidates.sort();
            self.deps.remove(&candidates[0]);
            Some(candidates[0])
        } else {
            None
        }
    }

    fn is_empty(&self) -> bool {
        self.deps.is_empty()
    }
}

#[derive(Clone, Copy, Debug)]
enum Worker {
    Working(char, usize),
    Free,
}

impl Worker {
    fn is_free(&self) -> bool {
        match *self {
            Worker::Free => true,
            _ => false,
        }
    }

    fn finish(&self) -> Option<usize> {
        match *self {
            Worker::Working(_, t) => Some(t),
            _ => None,
        }
    }
}

struct Scheduler {
    workers: [Worker; 5],
    step: usize,
}

impl Scheduler {
    fn new() -> Self {
        Self {
            workers: [Worker::Free; 5],
            step: 0,
        }
    }

    fn step(&self) -> usize {
        self.step
    }

    fn first_free_worker(&self) -> Option<usize> {
        self.workers
            .iter()
            .enumerate()
            .find(|(_, w)| w.is_free())
            .map(|(i, _)| i)
    }

    fn will_finish_first(&self) -> Option<usize> {
        self.workers
            .iter()
            .enumerate()
            .filter_map(|(i, w)| w.finish().map(|t| (i, t)))
            .min_by_key(|(_, t)| *t)
            .map(|(i, _)| i)
    }

    fn finish_task(&mut self) -> char {
        let first_to_finish = self.will_finish_first().unwrap();
        if let Worker::Working(task, step) = self.workers[first_to_finish] {
            self.step = step;
            self.workers[first_to_finish] = Worker::Free;
            task
        } else {
            panic!("first to finish not working");
        }
    }

    fn can_schedule_task(&self) -> bool {
        self.first_free_worker().is_some()
    }

    fn schedule_task(&mut self, task: char) {
        let first_free = self.first_free_worker().unwrap();
        let end_step = self.step + duration(task);
        assert!(self.workers[first_free].is_free());
        self.workers[first_free] = Worker::Working(task, end_step);
    }

    fn finish_all(&mut self) {
        let last_finish = self
            .workers
            .iter()
            .filter_map(|x| x.finish())
            .max()
            .unwrap();
        self.step = last_finish;
        self.workers = [Worker::Free; 5];
    }
}

fn duration(task: char) -> usize {
    let mut buf = [0u8];
    task.encode_utf8(&mut buf);
    buf[0] as usize - 4
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

    let mut scheduler = Scheduler::new();
    while !dependencies.is_empty() {
        loop {
            let next_step = dependencies.next_step();
            if let Some(next_task) = next_step {
                print!("{}", next_task);
                if !scheduler.can_schedule_task() {
                    let finished = scheduler.finish_task();
                    dependencies.satisfy_dep(finished);
                }
                scheduler.schedule_task(next_task);
                break;
            } else {
                let finished = scheduler.finish_task();
                dependencies.satisfy_dep(finished);
            }
        }
    }
    println!();

    scheduler.finish_all();

    println!("Part 2: {}", scheduler.step);
}
