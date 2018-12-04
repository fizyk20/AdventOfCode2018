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
        let mut max_guard = 0;
        let mut max_total = 0;
        for (&guard, &minutes) in &self.guards {
            let total = (&minutes).into_iter().cloned().sum();
            if total > max_total {
                max_guard = guard;
                max_total = total;
            }
        }
        max_guard
    }

    fn max_minute_for_guard(&self, guard: usize) -> usize {
        let minutes = self.guards.get(&guard).unwrap();
        let mut max_index = 0;
        let mut max_minutes = 0;
        for (i, &x) in minutes.into_iter().enumerate() {
            if x > max_minutes {
                max_index = i;
                max_minutes = x;
            }
        }
        max_index
    }

    fn max_guard_and_minute(&self) -> (usize, usize) {
        let mut max_guard = 0;
        let mut max_minute = 0;
        let mut max_minutes = 0;
        for &guard in self.guards.keys() {
            let minute = self.max_minute_for_guard(guard);
            let minutes = self.guards[&guard][minute];
            if minutes > max_minutes {
                max_minutes = minutes;
                max_minute = minute;
                max_guard = guard;
            }
        }
        (max_guard, max_minute)
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
