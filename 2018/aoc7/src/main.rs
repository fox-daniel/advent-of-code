use anyhow::Context;
use regex::Regex;
use std::fs;
use std::io::Write;
use std::ops::Deref;
use std::sync::LazyLock;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

static RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"Step (?<source>[A-Z]) must be finished before step (?<destination>[A-Z]) can begin.",
    )
    .expect("regex compiles")
});

fn main() -> Result<()> {
    let input = fs::read_to_string("input/input.txt")?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

fn part1(input: &str) -> Result<()> {
    let edges: Edges = input.parse()?;
    println!("{edges:#?}");
    Ok(())
}

#[derive(Debug)]
struct Edges(Vec<(char, char)>);

impl Deref for Edges {
    type Target = Vec<(char, char)>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::str::FromStr for Edges {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut edges = Vec::new();
        for line in s.split("\n") {
            if line.is_empty() {
                continue;
            }
            let cap = RE
                .captures(line)
                .with_context(|| format!("each trimmed line satisfies the regex {line}"))?;
            let source: Option<char> = cap
                .name("source")
                .and_then(|m| m.as_str().parse::<char>().map_or(None, |c| Some(c)));
            let destination: Option<char> = cap
                .name("destination")
                .and_then(|m| m.as_str().parse::<char>().map_or(None, |c| Some(c)));
            if let Some(s) = source && let Some(d) = destination {
                edges.push((s, d));
            }
        }
        Ok(Edges(edges))
    }
}
fn part2(input: &str) -> Result<()> {
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn test_regex() {
        let line = "Step X must be finished before step C can begin.";
        if let Some(cap) = RE.captures(line) {
            let source = cap.name("source").map(|m| Some(m.as_str()));
            let destination = cap.name("destination").map(|m| Some(m.as_str()));
            assert_eq!(source, Some(Some("X")));
            assert_eq!(destination, Some(Some("C")));
        }
    }
}
