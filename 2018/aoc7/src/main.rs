use anyhow::Context;
use regex::Regex;
use std::collections::{HashMap, HashSet, BinaryHeap};
use std::cmp::Reverse;
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
    // writeln!(std::io::stdout(), "{edges:#?}")?;
    let sequence: String = bfs(edges);    
    writeln!(std::io::stdout(), "{sequence:?}")?;    
    Ok(())
}


fn bfs(edges: Edges) -> String {
    let mut sequence = vec![];
    let mut destinations = HashSet::<char>::new();
    for destination_set in edges.0.values() {
        destinations.extend(destination_set);
    }
    let keys = HashSet::from_iter(edges.clone().into_keys());
    let roots: HashSet<Reverse<_>> = keys.difference(&destinations).cloned().map(|c| Reverse(c)).collect();
    // queue holds indegree=0 nodes in alphabetical order
    let mut queue = BinaryHeap::<Reverse<char>>::from_iter(roots);
    let mut indegree = HashMap::<char, u8>::new();
    edges.0.values().for_each(|v| {
        v.iter().for_each(|d| {let _ = indegree.entry(*d).and_modify(|count| *count +=1 ).or_insert(1);});
    });
    while !queue.is_empty() {
        let current = queue.pop().expect("cannot enter loop if queue is empty");
        if let Some(children) = edges.0.get(&current.0) {
            children.iter().for_each(|c| { let _ = indegree.entry(*c).and_modify(|v| *v -= 1);});
            let ready_set: HashSet<Reverse<char>> = children.iter().filter(|c| indegree.get(c).is_some_and(|v| *v == 0)).map(|c| Reverse(*c)).collect();
            queue.extend(ready_set);
        } 
        sequence.push(current);
    }
    sequence.iter().map(|c| c.0).collect()
}

#[derive(Debug)]
struct Edges(HashMap::<char, HashSet<char>>);

impl Deref for Edges {
    type Target = HashMap::<char, HashSet<char>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::str::FromStr for Edges {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut edges = HashMap::<char, HashSet<char>>::new();
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
                edges.entry(s).or_default().insert(d);
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

    #[test]
    fn test_example_input() {
        let input = "Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.";
        let edges: Edges = input.parse().unwrap();
        // writeln!(std::io::stdout(), "{edges:#?}").unwrap();
        let sequence: String = bfs(edges);    
        // writeln!(std::io::stdout(), "{sequence:?}")?;    
        assert_eq!(sequence, "CABDFE");        
    }
}
