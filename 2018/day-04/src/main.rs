use std::{collections::HashMap, fs};

use chrono::prelude::*;
use itertools::Itertools;

macro_rules! p {
    ($e:expr) => {{
        println!("{:#?}: {:#?}", stringify!($e), $e);
        $e
    }};
}

fn main() {
    let _example = r"`[1518-11-01 00:00] Guard #10 begins shift
    [1518-11-01 00:05] falls asleep
    [1518-11-01 00:25] wakes up
    [1518-11-01 00:30] falls asleep
    [1518-11-01 00:55] wakes up
    [1518-11-01 23:58] Guard #99 begins shift
    [1518-11-02 00:40] falls asleep
    [1518-11-02 00:50] wakes up
    [1518-11-03 00:05] Guard #10 begins shift
    [1518-11-03 00:24] falls asleep
    [1518-11-03 00:29] wakes up
    [1518-11-04 00:02] Guard #99 begins shift
    [1518-11-04 00:36] falls asleep
    [1518-11-04 00:46] wakes up
    [1518-11-05 00:03] Guard #99 begins shift
    [1518-11-05 00:45] falls asleep
    [1518-11-05 00:55] wakes up
    `"
    .to_owned();

    let _file = fs::read_to_string("input").unwrap();

    let records = get_lines(&_file)
        .iter()
        .map(Record::from)
        .sorted_by_key(|x| x.datetime)
        .collect_vec();

    // ---

    let min = records.first().unwrap().datetime;
    let max = records.last().unwrap().datetime;
    let relevant_datetimes = (min.timestamp()..max.timestamp())
        .map(|ts| NaiveDateTime::from_timestamp_opt(ts, 0).unwrap())
        .filter(|&dt| dt.second() == 0)
        .collect_vec();
    let mut active_guard_number = 0;
    let mut is_asleep = false;
    let mut asleep_by_datetime_by_guard = HashMap::new();
    for datetime in relevant_datetimes {
        if let Some(record) = records.iter().find(|r| r.datetime == datetime) {
            match record.event {
                Event::BeginsShift { guard_id } => {
                    active_guard_number = guard_id;
                    is_asleep = false;
                }
                Event::FallsAsleep() => {
                    is_asleep = true;
                }
                Event::WakesUp() => {
                    is_asleep = false;
                }
            };
        }

        asleep_by_datetime_by_guard
            .entry(active_guard_number)
            .or_insert_with(HashMap::new)
            // Now operating on the inner vec.
            .insert(datetime, is_asleep);
    }
    // p!(asleep_by_timestamp_by_guard);

    let minutes_asleep_by_guard = asleep_by_datetime_by_guard
        .iter()
        .map(|(&g, asleep_by_dt)| {
            (
                g,
                asleep_by_dt.iter().filter(|(_dt, &asleep)| asleep).count(),
            )
        })
        .collect_vec();
    // p!(&asleep);

    let guard_most_asleep = minutes_asleep_by_guard
        .iter()
        .max_by_key(|x| x.1)
        .unwrap()
        .0;
    // p!(&guard_most_asleep);

    let minutes_asleep_per_minute = asleep_by_datetime_by_guard
        .get(&guard_most_asleep)
        .unwrap()
        .iter()
        .filter(|(_, &asleep)| asleep)
        .map(|(&dt, _)| dt.minute())
        .sorted()
        .counts();
    // p!(&asleep_per_minute);

    let minute_most_asleep = minutes_asleep_per_minute
        .iter()
        .max_by_key(|(_minute, &count)| count)
        .unwrap()
        .0;
    // p!(&minute_most_asleep);

    let part_1a = guard_most_asleep * *minute_most_asleep;
    p!(part_1a);

    // ---

    let guard_and_minute = asleep_by_datetime_by_guard
        .iter()
        .flat_map(|(&g, hm)| {
            hm.iter()
                .filter(|(_dt, &asleep)| asleep)
                .map(move |(dt, _sl)| (g, dt.minute()))
                .collect_vec()
        })
        .collect_vec();
    // p!(&guard_and_minute);

    let counts = guard_and_minute.iter().counts();
    let guard_and_minute_most_asleep = counts.iter().max_by_key(|(_, count)| *count).unwrap().0;
    // p!(guard_and_minute_most_asleep);

    let part_2a = guard_and_minute_most_asleep.0 * guard_and_minute_most_asleep.1;
    p!(part_2a);
}

fn get_lines(s: &str) -> Vec<&str> {
    s.trim_matches('`')
        .split_terminator('\n')
        .map(|x| x.trim())
        .filter(|x| !x.is_empty())
        .collect_vec()
}

#[derive(Debug, Clone, Copy)]
struct Record {
    datetime: NaiveDateTime,
    event: Event,
}

impl From<&&str> for Record {
    fn from(s: &&str) -> Self {
        let (prefix, suffix) = s.split_at(18);
        let suffix = suffix.trim();

        let datetime = NaiveDateTime::parse_from_str(prefix, "[%Y-%m-%d %H:%M]").unwrap();
        let event = match suffix {
            "wakes up" => Event::WakesUp(),
            "falls asleep" => Event::FallsAsleep(),
            _ if suffix.ends_with("begins shift") => Event::BeginsShift {
                guard_id: suffix.split_terminator(' ').collect_vec()[1]
                    .trim_matches('#')
                    .parse()
                    .unwrap(),
            },
            _ => panic!(),
        };

        Record { datetime, event }
    }
}

#[derive(Debug, Clone, Copy)]
enum Event {
    BeginsShift { guard_id: u32 },
    FallsAsleep(),
    WakesUp(),
}
