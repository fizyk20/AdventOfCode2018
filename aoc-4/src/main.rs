#[macro_use]
extern crate nom;

use nom::{digit, line_ending, types::CompleteStr};
use std::cmp::{Ord, Ordering, PartialOrd};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Date {
    year: usize,
    month: usize,
    day: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Time {
    pub hour: usize,
    pub minute: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Event {
    StartShift(usize),
    WakeUp,
    FallAsleep,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct LogEntry {
    pub date: Date,
    pub time: Time,
    pub event: Event,
}

impl PartialOrd for LogEntry {
    fn partial_cmp(&self, other: &LogEntry) -> Option<Ordering> {
        if self.date == other.date {
            self.time.partial_cmp(&other.time)
        } else {
            self.date.partial_cmp(&other.date)
        }
    }
}

impl Ord for LogEntry {
    fn cmp(&self, other: &LogEntry) -> Ordering {
        if self.date == other.date {
            self.time.cmp(&other.time)
        } else {
            self.date.cmp(&other.date)
        }
    }
}

named!(
    date <CompleteStr, Date>, do_parse!(
        year: map!(digit, |x| x.parse().unwrap()) >>
        tag!("-") >>
        month: map!(digit, |x| x.parse().unwrap()) >>
        tag!("-") >>
        day: map!(digit, |x| x.parse().unwrap()) >>
        (Date { year, month, day })
    )
);

named!(
    time <CompleteStr, Time>, do_parse!(
        hour: map!(digit, |x| x.parse().unwrap()) >>
        tag!(":") >>
        minute: map!(digit, |x| x.parse().unwrap()) >>
        (Time { hour, minute })
    )
);

named!(
    datetime <CompleteStr, (Date, Time)>, do_parse!(
        date: date >>
        tag!(" ") >>
        time: time >>
        (date, time)
    )
);

named!(
    start_shift <CompleteStr, usize>, do_parse!(
        tag!("Guard #") >>
        id: digit >>
        tag!(" begins shift") >>
        (id.parse().unwrap())
    )
);

named!(
    event <CompleteStr, Event>, alt!(
        start_shift => { |x| Event::StartShift(x) } |
        tag!("wakes up") => { |_| Event::WakeUp } |
        tag!("falls asleep") => { |_| Event::FallAsleep }
    )
);

named!(
    entry <CompleteStr, LogEntry>, do_parse!(
        tag!("[") >>
        datetime: datetime >>
        tag!("] ") >>
        event: event >>
        line_ending >>
        (LogEntry { date: datetime.0, time: datetime.1, event })
    )
);

named!(input <CompleteStr, Vec<LogEntry>>, many1!(entry));

type Minutes = [usize; 60];

struct Analyser {
    current_guard: usize,
    fell_asleep: Option<Time>,
    guards: HashMap<usize, Minutes>,
}

impl Analyser {
    fn new() -> Self {
        Self {
            current_guard: 0,
            fell_asleep: None,
            guards: HashMap::new(),
        }
    }

    fn update(&mut self, entry: LogEntry) {
        match entry.event {
            Event::StartShift(id) => {
                self.current_guard = id;
            }
            Event::FallAsleep => {
                self.fell_asleep = Some(entry.time);
            }
            Event::WakeUp => {
                let start = self.fell_asleep.unwrap();
                let end = entry.time;
                self.fell_asleep = None;
                assert_eq!(start.hour, 0);
                assert_eq!(end.hour, 0);
                let guard_entry = self.guards.entry(self.current_guard).or_insert([0; 60]);
                for minute in start.minute..end.minute {
                    (*guard_entry)[minute] += 1;
                }
            }
        }
    }

    fn max_guard(&self) -> usize {
        self.guards
            .iter()
            .max_by_key(|(_, minutes)| minutes.into_iter().cloned().sum::<usize>())
            .map(|(&guard, _)| guard)
            .unwrap()
    }

    fn max_minute_for_guard(&self, guard: usize) -> usize {
        self.guards[&guard]
            .iter()
            .enumerate()
            .max_by_key(|(_, &x)| x)
            .map(|(i, _)| i)
            .unwrap()
    }

    fn max_guard_and_minute(&self) -> (usize, usize) {
        self.guards
            .iter()
            .flat_map(|(&guard, minutes)| {
                minutes
                    .into_iter()
                    .enumerate()
                    .map(move |(i, &mins)| (guard, i, mins))
            }).max_by_key(|(_, _, mins)| *mins)
            .map(|(guard, index, _)| (guard, index))
            .unwrap()
    }
}

fn main() {
    let mut file = File::open("input").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut log = input(CompleteStr(&contents)).unwrap().1;
    log.sort();

    let mut analyser = Analyser::new();
    for entry in log {
        analyser.update(entry);
    }

    let max_guard = analyser.max_guard();
    let max_minute = analyser.max_minute_for_guard(max_guard);

    println!(
        "Part 1:\nMax guard: {}\nMax minute: {}\nTotal: {}",
        max_guard,
        max_minute,
        max_guard * max_minute
    );

    let (max_guard, max_minute) = analyser.max_guard_and_minute();

    println!(
        "Part 2:\nMax guard: {}\nMax minute: {}\nTotal: {}",
        max_guard,
        max_minute,
        max_guard * max_minute
    );
}
