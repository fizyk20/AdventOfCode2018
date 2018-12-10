use nom::types::CompleteStr;
use nom::*;
use std::fs::File;
use std::io::Read;
use std::isize;

named!(number <CompleteStr, isize>, map!(digit, |x| x.parse().unwrap()));

named!(sign <CompleteStr, isize>, alt!(
    char!(' ') => { |_| 1 } |
    char!('-') => { |_| -1 }
));

named!(signed <CompleteStr, isize>, do_parse!(
    sign: sign >>
    num: number >>
    (sign * num)
));

#[derive(Clone, Copy, Debug)]
struct Light {
    pos: (isize, isize),
    vel: (isize, isize),
}

impl Light {
    fn step(&mut self) {
        self.pos.0 += self.vel.0;
        self.pos.1 += self.vel.1;
    }

    fn step_rev(&mut self) {
        self.pos.0 -= self.vel.0;
        self.pos.1 -= self.vel.1;
    }
}

named!(light <CompleteStr, Light>, do_parse!(
    tag!("position=<") >>
    pos_x: signed >>
    tag!(", ") >>
    pos_y: signed >>
    tag!("> velocity=<") >>
    vel_x: signed >>
    tag!(", ") >>
    vel_y: signed >>
    tag!(">") >>
    multispace >>
    (Light { pos: (pos_x, pos_y), vel: (vel_x, vel_y) })
));

named!(parse <CompleteStr, Vec<Light>>, many1!(light));

struct Sky {
    lights: Vec<Light>,
    bbox: BBox,
}

struct BBox {
    min_x: isize,
    max_x: isize,
    min_y: isize,
    max_y: isize,
}

impl Sky {
    fn new(lights: Vec<Light>) -> Self {
        let max_x = lights.iter().map(|l| l.pos.0).max().unwrap();
        let max_y = lights.iter().map(|l| l.pos.1).max().unwrap();
        let min_x = lights.iter().map(|l| l.pos.0).min().unwrap();
        let min_y = lights.iter().map(|l| l.pos.1).min().unwrap();

        Self {
            lights,
            bbox: BBox {
                min_x,
                min_y,
                max_x,
                max_y,
            },
        }
    }

    fn step(&mut self) {
        let mut max_x = isize::MIN;
        let mut max_y = isize::MIN;
        let mut min_x = isize::MAX;
        let mut min_y = isize::MAX;

        for light in &mut self.lights {
            light.step();
            if light.pos.0 > max_x {
                max_x = light.pos.0;
            }
            if light.pos.1 > max_y {
                max_y = light.pos.1;
            }
            if light.pos.0 < min_x {
                min_x = light.pos.0;
            }
            if light.pos.1 < min_y {
                min_y = light.pos.1;
            }
        }

        self.bbox = BBox {
            min_x,
            min_y,
            max_x,
            max_y,
        };
    }

    fn step_rev(&mut self) {
        let mut max_x = isize::MIN;
        let mut max_y = isize::MIN;
        let mut min_x = isize::MAX;
        let mut min_y = isize::MAX;

        for light in &mut self.lights {
            light.step_rev();
            if light.pos.0 > max_x {
                max_x = light.pos.0;
            }
            if light.pos.1 > max_y {
                max_y = light.pos.1;
            }
            if light.pos.0 < min_x {
                min_x = light.pos.0;
            }
            if light.pos.1 < min_y {
                min_y = light.pos.1;
            }
        }

        self.bbox = BBox {
            min_x,
            min_y,
            max_x,
            max_y,
        };
    }

    fn bbox_area(&self) -> isize {
        (self.bbox.max_x - self.bbox.min_x + 1) * (self.bbox.max_y - self.bbox.min_y + 1)
    }

    fn print(&self) {
        let row_len = self.bbox.max_x - self.bbox.min_x + 1;
        let mut lines = (self.bbox.min_y..=self.bbox.max_y)
            .map(|_| vec![b' '; row_len as usize])
            .collect::<Vec<_>>();

        for light in &self.lights {
            let row = (light.pos.1 - self.bbox.min_y) as usize;
            let col = (light.pos.0 - self.bbox.min_x) as usize;
            lines[row][col] = b'#';
        }

        for line in lines {
            let s = String::from_utf8(line).unwrap();
            println!("{}", s);
        }
    }
}

fn main() {
    let mut file = File::open("input").unwrap();
    let mut content = String::new();
    let _ = file.read_to_string(&mut content);

    let data = parse(CompleteStr(&content)).unwrap().1;

    let mut sky = Sky::new(data);
    let mut bbox_area = sky.bbox_area();
    let mut seconds = 0;

    loop {
        sky.step();
        if sky.bbox_area() > bbox_area {
            sky.step_rev();
            break;
        }
        bbox_area = sky.bbox_area();
        seconds += 1;
    }

    println!("Part 1:");
    sky.print();

    println!("Part 2: {}", seconds);
}
