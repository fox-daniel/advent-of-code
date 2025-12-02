use anyhow::Context;
use chrono::{DateTime, NaiveDateTime, Timelike, Utc};
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use std::sync::LazyLock;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string("input/input.txt")?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

#[derive(Debug)]
struct Record {
    datetime: DateTime<Utc>,
    sleep: Option<bool>,
    id: Option<u32>,
}

static RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\[(?<datetime>\d\d\d\d-\d\d-\d\d \d\d:\d\d)\] (?:.*(?<sleep>asleep|wakes)|Guard #(?<id>\d+))")
        .expect("regex compiles")
});

impl std::str::FromStr for Record {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cap = RE
            .captures(s)
            .with_context(|| format!("failed to capture regex from:{s}"))?;
        let id = cap
            .name("id")
            .and_then(|m| m.as_str().parse::<u32>().map_or(None, |i| Some(i)));
        let sleep = cap.name("sleep").map(|m| m.as_str() == "asleep");
        let naive = NaiveDateTime::parse_from_str(&cap["datetime"], "%Y-%m-%d %H:%M")?;
        let datetime = DateTime::<Utc>::from_naive_utc_and_offset(naive, Utc);
        Ok(Self {
            datetime,
            sleep,
            id,
        })
    }
}

fn part1(input: &str) -> Result<(), Box<dyn std::error::Error>> {
    // create Vec<Record>
    // O(nlog(n)): sort entries by chronological by datetime; use a datetime library
    // O(n): create counts of sleep time for each guard, put in HashMap<GuardId, Count>
    // O(# guards): find guard that slept longest by using an iter fold
    // chain the next two steps:
    // O(n): filter datetime to only include single guard
    // O(n): using just time, not date, create hashmap<Minute, SleepCount>
    // O(n): iter fold to get max entry
    // id * minute
    let mut records = Vec::<Record>::new();
    for line in input.lines() {
        records.push(line.parse()?);
    }

    sort_records(&mut records);
    println!("{records:#?}");
    let mut sleep_counts = HashMap::<u32, u32>::new();
    let mut current_id = records[0].id.unwrap();
    let mut sleep_start: DateTime<Utc> = records[0].datetime;
    for record in records.iter() {
        if record.id.is_some() && (record.id.unwrap() != current_id) {
            current_id = record.id.unwrap();
            continue;
        }
        if record.sleep.is_some() && record.sleep.unwrap() {
            sleep_start = record.datetime;
            continue;
        }
        if record.sleep.is_some() && !record.sleep.unwrap() {
            let duration = record
                .datetime
                .signed_duration_since(sleep_start)
                .num_minutes();
            let duration = duration as u32;
            sleep_counts
                .entry(current_id)
                .and_modify(|c| *c += duration)
                .or_insert(duration);
        }
    }
    println!("{:#?}", sleep_counts);
    let (id, _) = sleep_counts.iter().fold((u32::MAX, 0), |acc, item| {
        if *item.1 > acc.1 {
            (*item.0, *item.1)
        } else {
            acc
        }
    });
    println!("{id}");
    // in order to filter, one must check if the most recent id matches the target id
    // current_id = records[0].id.unwrap();
    records.retain_mut(|record| {
        if record.id.is_some() {
            current_id = record.id.unwrap();
        }
        current_id == id
    });
    // println!("{records:#?}");
    let mut minute_counts = HashMap::<u32, u32>::new();
    let mut sleep_start_minute = records[1].datetime.minute();
    let mut sleep_stop_minute = records[2].datetime.minute();
    for record in records.iter() {
        if record.sleep.is_some_and(|sleep| sleep) {
            sleep_start_minute = record.datetime.minute();
            continue;
        }
        if record.sleep.is_some_and(|sleep| !sleep) {
            sleep_stop_minute = record.datetime.minute();
            for minute in sleep_start_minute..sleep_stop_minute {
                minute_counts
                    .entry(minute)
                    .and_modify(|c| *c += 1)
                    .or_insert(1);
            }
        }
    }
    let max_item = minute_counts
        .iter()
        .max_by_key(|item| *item.1)
        .expect("obtain max minute by count");
    writeln!(io::stdout(), "max minute: {}", max_item.0).ok();
    writeln!(io::stdout(), "answer: {}", max_item.0 * id).ok();
    Ok(())
}

fn sort_records(records: &mut [Record]) {
    records.sort_by_key(|r| r.datetime);
}

fn part2(input: &str) -> std::io::Result<()> {
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use chrono::{NaiveDate, NaiveTime};
    const INPUT_LINES: &str = "[1518-04-16 00:46] falls asleep
    [1518-10-31 00:46] wakes up
    [1518-10-14 00:02] Guard #2459 begins shift";
    #[test]
    fn parse_sleep_record_test() -> Result<(), Box<dyn std::error::Error + 'static>> {
        let line = INPUT_LINES.lines().next().unwrap();
        let record: Record = line.parse()?;
        assert!(record.sleep.unwrap());
        Ok(())
    }

    #[test]
    fn parse_id_record_test() -> Result<(), Box<dyn std::error::Error + 'static>> {
        let line = INPUT_LINES.lines().nth(2).unwrap();
        let record: Record = line.parse()?;
        assert_eq!(record.id.unwrap(), 2459u32);
        Ok(())
    }

    #[test]
    fn parse_wake_record_test() -> Result<(), Box<dyn std::error::Error + 'static>> {
        let line = INPUT_LINES.lines().nth(1).unwrap();
        let record: Record = line.parse()?;
        assert!(!record.sleep.unwrap());
        Ok(())
    }

    #[test]
    fn sort_records_test() -> Result<(), Box<dyn std::error::Error + 'static>> {
        let mut records = Vec::<Record>::new();
        for line in INPUT_LINES.lines() {
            records.push(line.parse()?)
        }
        sort_records(&mut records);
        assert_eq!(
            records[0].datetime,
            DateTime::<Utc>::from_naive_utc_and_offset(
                NaiveDateTime::new(
                    NaiveDate::from_ymd_opt(1518, 4, 16).expect("valid date"),
                    NaiveTime::from_hms_opt(0, 46, 0).expect("valide hour")
                ),
                Utc
            )
        );
        assert_eq!(
            records[1].datetime,
            DateTime::<Utc>::from_naive_utc_and_offset(
                NaiveDateTime::new(
                    NaiveDate::from_ymd_opt(1518, 10, 14).expect("valid date"),
                    NaiveTime::from_hms_opt(0, 2, 0).expect("valid time"),
                ),
                Utc
            )
        );
        Ok(())
    }
}
