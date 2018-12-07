use nom::types::CompleteStr;
use nom::*;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Read;
use std::mem;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct Point {
    pub x: isize,
    pub y: isize,
}

impl Point {
    fn left(self) -> Point {
        Point {
            x: self.x - 1,
            y: self.y,
        }
    }

    fn right(self) -> Point {
        Point {
            x: self.x + 1,
            y: self.y,
        }
    }

    fn up(self) -> Point {
        Point {
            x: self.x,
            y: self.y - 1,
        }
    }

    fn down(self) -> Point {
        Point {
            x: self.x,
            y: self.y + 1,
        }
    }
}

fn distance(p1: Point, p2: Point) -> usize {
    ((p1.x - p2.x).abs() + (p1.y - p2.y).abs()) as usize
}

named!(point <CompleteStr, Point>, do_parse!(
    x: map!(digit, |n| n.parse().unwrap()) >>
    tag!(", ") >>
    y: map!(digit, |n| n.parse().unwrap()) >>
    line_ending >>
    (Point { x, y })
));

named!(parse <CompleteStr, Vec<Point>>, many1!(point));

type CenterId = u8;

enum Area {
    Finite(Vec<Point>),
    Infinite,
}

struct Canvas {
    points: HashMap<Point, CenterId>,
    owners: HashMap<CenterId, Area>,
}

impl Canvas {
    fn new() -> Self {
        Self {
            points: HashMap::new(),
            owners: HashMap::new(),
        }
    }

    fn update_for_point(&mut self, center_id: CenterId, centers: &[Point]) {
        let center = centers[center_id as usize];
        let mut points_queue = HashSet::new();
        points_queue.insert(center);

        'outer: while !points_queue.is_empty() {
            let queue = mem::replace(&mut points_queue, HashSet::new());
            for point in queue {
                if point.x < 0 || point.y < 0 || point.x > 500 || point.y > 500 {
                    let _ = self.owners.insert(center_id, Area::Infinite);
                    break 'outer;
                }
                self.points.insert(point, center_id);
                if let Area::Finite(ref mut v) = *self
                    .owners
                    .entry(center_id)
                    .or_insert_with(|| Area::Finite(Vec::new()))
                {
                    v.push(point);
                }
                for new_point in vec![point.left(), point.right(), point.up(), point.down()] {
                    if self.points.contains_key(&new_point) {
                        continue;
                    }
                    if !is_closest(new_point, centers, center_id) {
                        continue;
                    }
                    points_queue.insert(new_point);
                }
            }
        }
    }

    fn max_size(&self) -> usize {
        self.owners
            .values()
            .filter_map(|area| match area {
                Area::Finite(v) => Some(v.len()),
                _ => None,
            })
            .max()
            .unwrap()
    }
}

fn is_closest(point: Point, centers: &[Point], center_id: CenterId) -> bool {
    let dist = distance(point, centers[center_id as usize]);
    !centers
        .into_iter()
        .enumerate()
        .filter(|&(i, _)| i != center_id as usize)
        .any(|(_, p)| distance(point, *p) <= dist)
}

fn main() {
    let mut file = File::open("input").unwrap();
    let mut input = String::new();
    let _ = file.read_to_string(&mut input);

    let points = parse(CompleteStr(&input)).unwrap().1;

    let mut canvas = Canvas::new();

    for point_id in 0..points.len() {
        println!("Analysing #{}", point_id);
        canvas.update_for_point(point_id as CenterId, &points);
    }

    println!("Part 1: {}", canvas.max_size());

    let mut area = 0;
    for x in 0..500 {
        for y in 0..500 {
            let point = Point { x, y };
            let sum_distances = points.iter().map(|p| distance(*p, point)).sum::<usize>();
            if sum_distances < 10000 {
                area += 1;
            }
        }
    }

    println!("Part 2: {}", area);
}
