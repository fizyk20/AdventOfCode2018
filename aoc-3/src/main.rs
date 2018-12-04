#[macro_use]
extern crate nom;

use nom::types::CompleteStr;
use nom::{digit, line_ending};
use std::fs::File;
use std::io::Read;

named!(
    bare_num <CompleteStr, usize>, map!(digit, |x| x.parse().unwrap())
);

#[derive(Debug, Clone, Copy)]
struct Claim {
    pub id: usize,
    pub x: usize,
    pub y: usize,
    pub w: usize,
    pub h: usize,
}

named!(claim <CompleteStr, Claim>, do_parse!(
        tag!("#") >>
        id: bare_num >>
        tag!(" @ ") >>
        x: bare_num >>
        tag!(",") >>
        y: bare_num >>
        tag!(": ") >>
        w: bare_num >>
        tag!("x") >>
        h: bare_num >>
        line_ending >>
        (Claim { id, x, y, w, h })
    )
);

named!(input <CompleteStr, Vec<Claim>>, many1!(claim));

const DIM: usize = 1000;

fn coord_to_index(x: usize, y: usize) -> usize {
    x * DIM + y
}

fn apply_claim(canvas: &mut [u8], claim: Claim) {
    for x in claim.x..claim.x + claim.w {
        for y in claim.y..claim.y + claim.h {
            canvas[coord_to_index(x, y)] += 1;
        }
    }
}

fn main() {
    let mut file = File::open("input").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let claims = input(CompleteStr(&contents)).unwrap();

    let mut canvas = [0u8; DIM * DIM];
    for claim in claims.1 {
        apply_claim(&mut canvas, claim);
    }

    println!(
        "Part 1: {}",
        (&canvas).into_iter().filter(|&&x| x > 1).count()
    );
}
