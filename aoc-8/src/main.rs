use nom::types::CompleteStr;
use nom::*;
use std::fs::File;
use std::io::Read;

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

fn main() {
    let mut file = File::open("input").unwrap();
    let mut data = String::new();
    let _ = file.read_to_string(&mut data);
    data.trim_right();

    let data = parse(CompleteStr(&data)).unwrap().1;

    println!("Part 1: {}", data.sum_all_metadata());
    println!("Part 2: {}", data.value());
}
