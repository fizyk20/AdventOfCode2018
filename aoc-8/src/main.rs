use nom::types::CompleteStr;
use nom::*;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{Read, Write};

#[derive(Clone, Debug)]
struct TreeNode {
    children: Vec<TreeNode>,
    metadata: Vec<usize>,
}

named!(number <CompleteStr, usize>, do_parse!(
    num: map!(digit, |x| x.parse().unwrap()) >>
    multispace >>
    (num)
));

named!(parse <CompleteStr, TreeNode>, do_parse!(
    num_children: number >>
    num_metadata: number >>
    children: count!(parse, num_children) >>
    metadata: count!(number, num_metadata) >>
    (TreeNode { children, metadata })
));

impl TreeNode {
    fn sum_all_metadata(&self) -> usize {
        self.metadata.iter().cloned().sum::<usize>()
            + self
                .children
                .iter()
                .map(|c| c.sum_all_metadata())
                .sum::<usize>()
    }

    fn value(&self) -> usize {
        if self.children.is_empty() {
            self.sum_all_metadata()
        } else {
            self.metadata
                .iter()
                .filter(|&x| *x != 0)
                .filter_map(|x| self.children.get(*x - 1))
                .map(|c| c.value())
                .sum::<usize>()
        }
    }
}

fn dump(node: &TreeNode) {
    let mut file = File::create("graph.dot").unwrap();
    let mut data = String::new();
    data.push_str("digraph aoc8 {\n");

    let mut node_queue = VecDeque::new();
    node_queue.push_back((1usize, node));
    let mut next_id = 2;

    while !node_queue.is_empty() {
        let (cur_id, cur_node) = node_queue.pop_front().unwrap();
        data.push_str(&format!(
            "{} [label=\"{}\nMetadata: {:?}\"]\n",
            cur_id, cur_id, cur_node.metadata
        ));
        for child in &cur_node.children {
            node_queue.push_back((next_id, child));
            data.push_str(&format!("{} -> {}\n", cur_id, next_id));
            next_id += 1;
        }
    }

    data.push_str("}\n");

    let _ = file.write_all(data.as_bytes());
}

fn main() {
    let mut file = File::open("input").unwrap();
    let mut data = String::new();
    let _ = file.read_to_string(&mut data);
    data.trim_right();

    let data = parse(CompleteStr(&data)).unwrap().1;

    println!("Part 1: {}", data.sum_all_metadata());
    println!("Part 2: {}", data.value());

    dump(&data);
}
