#[macro_use]
extern crate nom;

use nom::{digit, line_ending, types::CompleteStr};
use std::collections::HashSet;
use std::fs::File;
use std::io::Read;

named!(
    sign <CompleteStr, i32>, alt!(
        tag!("+") => { |_| 1 } |
        tag!("-") => { |_| -1 }
    )
);

named!(
    line <CompleteStr, i32>, do_parse!(
        sgn: sign >>
        bare_num: digit >>
        line_ending >>
        (sgn * bare_num.parse::<i32>().unwrap())
    )
);

named!(input <CompleteStr, Vec<i32>>, many1!(line));

fn main() {
    let mut file = File::open("input").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let values = input(CompleteStr(&contents)).unwrap().1;

    let sum = values.iter().cloned().sum::<i32>();
    println!("Part1: {}", sum);

    let mut encountered = HashSet::new();
    let _ = encountered.insert(0);
    let mut current_freq = 0;

    for freq in values.into_iter().cycle() {
        current_freq += freq;
        if encountered.contains(&current_freq) {
            break;
        }
        let _ = encountered.insert(current_freq);
    }
    println!("Part2: {}", current_freq);
}
