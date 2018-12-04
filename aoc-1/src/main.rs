#[macro_use]
extern crate nom;

use nom::{digit, line_ending, types::CompleteStr};
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

    let sum = values.into_iter().sum::<i32>();
    println!("Part1: {}", sum);
}
