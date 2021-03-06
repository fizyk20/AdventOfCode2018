use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

struct Counter {
    is_exactly_two: usize,
    is_exactly_three: usize,
}

impl Counter {
    fn new() -> Self {
        Self {
            is_exactly_two: 0,
            is_exactly_three: 0,
        }
    }

    fn update(&mut self, id: &str) {
        let mut counts = HashMap::new();
        for letter in id.chars() {
            let entry = counts.entry(letter).or_insert(0);
            *entry += 1;
        }
        if counts.values().any(|v| *v == 2) {
            self.is_exactly_two += 1;
        }
        if counts.values().any(|v| *v == 3) {
            self.is_exactly_three += 1;
        }
    }

    fn checksum(&self) -> usize {
        self.is_exactly_two * self.is_exactly_three
    }
}

fn difference(str1: &str, str2: &str) -> usize {
    str1.chars()
        .zip(str2.chars())
        .filter(|&(a, b)| a != b)
        .count()
}

fn main() {
    let mut file = File::open("input").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut counter = Counter::new();
    for line in contents.lines() {
        counter.update(line);
    }

    println!("Part 1: {}", counter.checksum());

    'outer: for str1 in contents.lines() {
        for str2 in contents.lines() {
            if difference(str1, str2) == 1 {
                println!("Part 2:\n{}\n{}", str1, str2);
                break 'outer;
            }
        }
    }
}
