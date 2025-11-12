use std::collections::HashMap;
use std::fs;
use std::io::Write;

fn main() -> std::io::Result<()> {
    let input = fs::read_to_string("input/input.txt")?;

    part1(&input)?;
    // part2(&input)?;

    Ok(())
}

fn part1(input: &str) -> std::io::Result<()> {
    let mut doubles: u32 = 0;
    let mut triples: u32 = 0;
    let mut counts = HashMap::<char, u32>::new();
    for line in input.lines() {
        counts.clear();
        for char in line.chars() {
            counts
                .entry(char)
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
        }
        let (has_double, has_triple) = counts.values().fold((false, false), |acc, v| {
            if *v == 2 {
                (true, acc.1)
            } else if *v == 3 {
                (acc.0, true)
            } else {
                acc
            }
        });
        if has_double {
            doubles += 1;
        };
        if has_triple {
            triples += 1;
        };
    }
    let check_sum = doubles * triples;
    let check_sum = format!("{check_sum}\n");
    std::io::stdout().write_all(check_sum.as_bytes())?;
    Ok(())
}

fn part2(input: &str) -> std::io::Result<()> {
    // O(N^2*K): measure the distance between all strings; use early stop
    Ok(())
}

fn off_by_one(s1: &str, s2: &str) -> bool {
    let mut dist = 0;
    for (c1, c2) in s1.chars().zip(s2.chars()) {
        if c1 != c2 {
            dist += 1
        }
        if dist > 1 {
            return false;
        }
    }
    dist == 1
}

mod test {
    use super::*;
    #[test]
    fn test_off_by_one() {
        let s1 = "abcd";
        let s2 = "abce";
        assert!(off_by_one(s1, s2));
    }
    #[test]
    fn test_not_off_by_one() {
        let s1 = "abcd";
        let s2 = "abfe";
        assert!(!off_by_one(s1, s2));
    }
}
