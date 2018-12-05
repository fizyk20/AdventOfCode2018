use std::fs::File;
use std::io::Read;

fn check_opposite(c1: char, c2: char) -> bool {
    c1.to_lowercase().next().unwrap() == c2.to_lowercase().next().unwrap() && c1 != c2
}

fn append(new_str: &mut Vec<char>, c: char) {
    if new_str.len() > 0 {
        let last_index = new_str.len() - 1;
        let last_c = new_str[last_index];
        if check_opposite(last_c, c) {
            new_str.truncate(last_index);
        } else {
            new_str.push(c);
        }
    } else {
        new_str.push(c);
    }
}

fn collapse<I: IntoIterator<Item = char>>(data: I) -> Vec<char> {
    let mut result = Vec::new();
    for c in data {
        append(&mut result, c);
    }
    result
}

fn main() {
    let mut file = File::open("input").unwrap();
    let mut contents = String::new();
    let _ = file.read_to_string(&mut contents);
    let initial_data: Vec<char> = contents.trim_end().chars().collect();

    let collapsed_data = collapse(initial_data.clone());

    println!("Part 1: {}", collapsed_data.len());

    let min_len = (97u8..=122) // 'a' to 'z'
        .map(|exclude| {
            let filtered_data = initial_data
                .iter()
                .filter(|&c| c.to_lowercase().next().unwrap() != exclude.into())
                .cloned();
            collapse(filtered_data).len()
        }).min()
        .unwrap();

    println!("Part 2: {}", min_len);
}
