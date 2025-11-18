use regex::Regex;
use std::fs;
use std::io;
use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string("input/input.txt")?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

#[derive(Debug)]
struct Claim {
    id: u32,
    left_edge: u32,
    right_edge: u32,
    width: u32,
    length: u32,
}

impl Claim {
    fn from_capture(c: regex::Captures) -> Self {
        Claim {
            id: c["id"].parse::<u32>().unwrap(),
            left_edge: c["left_edge"].parse::<u32>().unwrap(),
            right_edge: c["right_edge"].parse::<u32>().unwrap(),
            width: c["width"].parse::<u32>().unwrap(),
            length: c["length"].parse::<u32>().unwrap(),
        }
    }
}

fn part1(input: &str) -> Result<(), Box<dyn std::error::Error>> {
    // brute force:
    // 1. scan through data to determine max coordinates in each dimension: O(k)
    // 2. double for loop over dimensions; for each iteration check if each id covers that and update a dict: O(k*n^2)
    // 3. iterate over dict and count entries where val is >=2
    //
    // Potential performance improvements: use an r-tree type structure
    // - divide into disjoint groups with a union find: cons: it is possible that they will all be in one group;
    // - divide grid into chunks and create a lookup of which id's have claims that overlap that chunk;
    // for each chunk use brute force. how to chunk?

    let mut claims: Vec<Claim> = vec![];
    let re = Regex::new(
        r"#(?<id>\d+) @ (?<left_edge>\d+),(?<right_edge>\d+): (?<width>\d+)x(?<length>\d+)",
    )?;
    let mut claim: Claim;
    for line in input.lines() {
        claim = re
            .captures_iter(line)
            .map(|c| Claim::from_capture(c))
            .next()
            .unwrap();
        claims.push(claim);
    }
    writeln!(io::stdout(), "{:#?}", claims[..3])?;
    Ok(())
}

fn part2(input: &str) -> std::io::Result<()> {
    Ok(())
}
