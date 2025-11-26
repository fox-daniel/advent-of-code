use anyhow::Context;
use chrono::{DateTime, NaiveDateTime, Utc};
use regex::Regex;
use std::collections::HashMap;
use std::fs;
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

    records.sort_by_key(|r| r.datetime);
    // println!("{:#?}", &records[..3]);

    let mut sleep_counts = HashMap::<u32, u32>::new();
    let mut current_id = u32::MAX;
    for record in records.iter() {
        if record.id.is_some() && (record.id.unwrap() != current_id) {
            current_id = record.id.unwrap();
        }
        if record.sleep.is_some() && record.sleep.unwrap() {
            sleep_counts
                .entry(current_id)
                .and_modify(|c| *c += 1)
                .or_insert(1);
        }
    }
    // println!("{:#?}", sleep_counts);
    let (id, _) = sleep_counts.iter().fold((u32::MAX, 0), |acc, item| {
        if *item.1 > acc.1 {
            (*item.0, *item.1)
        } else {
            acc
        }
    });
    println!("{id}");
    // in order to filter, one must check if the most recent id matches the target id
    Ok(())
}

fn part2(input: &str) -> std::io::Result<()> {
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
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
}
