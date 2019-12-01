use std::io::BufRead;
use std::io;

use std::collections::HashMap;

use std::cmp::Ordering;
use std::fmt;
use std::fs;

use regex::Regex;

use lazy_static;

enum LogEntry {
    FallsAsleep,
    WakesUp,
    ShiftChange(usize),
}

struct Log {
    y: u32,
    m: u32,
    d: u32,
    hh: u32,
    mm: u32,
    entry: LogEntry,
}

impl Ord for Log {
    fn cmp(&self, other: &Self) -> Ordering {
        self.y.cmp(&other.y)
            .then_with(|| self.m.cmp(&other.m))
            .then_with(|| self.d.cmp(&other.d))
            .then_with(|| self.hh.cmp(&other.hh))
            .then_with(|| self.mm.cmp(&other.mm))
    }
}

impl PartialOrd for Log {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Log {
    fn eq(&self, other: &Self) -> bool {
        self.y == other.y && 
        self.m == other.m && 
        self.d == other.d && 
        self.hh == other.hh && 
        self.mm == other.mm
    }
}

impl Eq for Log {}

impl fmt::Display for Log {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{:04}-{:02}-{:02} {:02}:{:02}] ", 
            self.y, self.m, self.d, self.hh, self.mm)?;
        match self.entry {
            LogEntry::FallsAsleep => 
                write!(f, "falls asleep"),
            LogEntry::WakesUp => 
                write!(f, "wakes up"),
            LogEntry::ShiftChange(id) => 
                write!(f, "Guard #{} begins shift", id),
        }
    }
}

fn read_inputs(filename: String) -> io::Result<Vec<Log>> {
    let file_in = fs::File::open(filename)?;
    let file_reader = io::BufReader::new(file_in);
    Ok(file_reader.lines().filter_map(io::Result::ok).map(|line| -> Log {
        lazy_static! {
            static ref LINE_RE: Regex = Regex::new(r"\[(\d{4})-(\d{2})-(\d{2}) (\d{2}):(\d{2})\] (.*)").unwrap();
        }
        if LINE_RE.is_match(&line) {
            for line_cap in LINE_RE.captures_iter(&line) {
                let y: u32 = line_cap[1].parse().unwrap();
                let m: u32 = line_cap[2].parse().unwrap();
                let d: u32 = line_cap[3].parse().unwrap();
                let hh: u32 = line_cap[4].parse().unwrap();
                let mm: u32 = line_cap[5].parse().unwrap();

                let entry = &line_cap[6];

                if entry.cmp("falls asleep") == Ordering::Equal {
                    return Log { y: y, m: m, d: d, hh: hh, mm: mm, entry: LogEntry::FallsAsleep };
                }
                if entry.cmp("wakes up") == Ordering::Equal {
                    return Log { y: y, m: m, d: d, hh: hh, mm: mm, entry: LogEntry::WakesUp };
                }

                lazy_static! {
                    static ref GUARD_RE: Regex = Regex::new(r"Guard #(\d*) begins shift").unwrap();
                }
                if GUARD_RE.is_match(&entry) {
                    for guard_cap in GUARD_RE.captures_iter(&entry) {
                        let id: usize = guard_cap[1].parse().unwrap();
                        return Log { y: y, m: m, d: d, hh: hh, mm: mm, entry: LogEntry::ShiftChange(id) };
                    }
                    panic!("Invalid input");
                }
                panic!("Invalid input");
            }
            panic!("Invalid input");
        }
        panic!("Invalid input");
    }).collect())
}

pub fn run() {
    let mut inputs = read_inputs("data/day04.txt".to_string())
        .expect("Can't read file");
    inputs.sort();

    let mut active_guard = 0;
    let mut fell_asleep = 0;

    let mut sleep_logs = HashMap::new();

    for input in inputs {
        match input.entry {
            LogEntry::ShiftChange(id) => {
                active_guard = id
            },
            LogEntry::FallsAsleep => {
                fell_asleep = input.mm;
            },
            LogEntry::WakesUp => {
                let guard_sleep_log = sleep_logs.entry(active_guard).or_insert([0; 60]);

                let woke_up = input.mm;
                for mm in fell_asleep..woke_up {
                    guard_sleep_log[mm as usize] += 1;
                } 
            }
        }
    }

    // Part 1
    let (max_id, _) = sleep_logs.iter()
        .map(|(id, logs)| {
            let total_sleep : u32 = logs.iter().sum();
            (id, total_sleep)
        })
        .max_by_key(|(_, total_sleep)| *total_sleep).unwrap();
    
    let (max_mm, _) = sleep_logs[&max_id].iter()
        .enumerate()
        .max_by_key(|(_, sleep)| *sleep).unwrap();

    println!("{} * {} = {}", max_id, max_mm, max_id * max_mm);


    // Part 2
    let (max_id, max_mm, _) = sleep_logs.iter()
        .map(|(id, logs)| {
            let (max_mm, max_sleep) = logs.iter()
                .enumerate()    
                .max_by_key(|(_, sleep)| *sleep).unwrap();
            (id, max_mm, max_sleep)
        }).max_by_key(|(_, _, sleep)| *sleep).unwrap();
    
    println!("{} * {} = {}", max_id, max_mm, max_id * max_mm);
}