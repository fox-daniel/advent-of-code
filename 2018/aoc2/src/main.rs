use std::collections::HashMap;
use std::fs;
use std::io::Write;

fn main() -> std::io::Result<()> {
    let input = fs::read_to_string("input/input.txt")?;

    part1(&input)?;
    part2(&input)?;

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
        if counts.values().any(|v| *v == 2) {
            doubles += 1;
        };
        if counts.values().any(|v| *v == 3) {
            triples += 1;
        }
    }
    let check_sum = doubles * triples;
    let check_sum = format!("{check_sum}\n");
    std::io::stdout().write_all(check_sum.as_bytes())?;
    Ok(())
}

fn part2(input: &str) -> std::io::Result<()> {
    // O(N^2*K): measure the distance between all strings; use early stop
    let mut id1: String = String::new();
    let mut id2: String = String::new();
    for s1 in input.lines() {
        for s2 in input.lines() {
            if off_by_one(s1, s2) {
                id1 = s1.to_string();
                id2 = s2.to_string();
                break;
            }
        }
    }
    let id: String = id1
        .chars()
        .zip(id2.chars())
        .filter(|(c1, c2)| c1 == c2)
        .map(|item| item.0)
        .collect();
    println!("{id}");
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

#[cfg(test)]
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
