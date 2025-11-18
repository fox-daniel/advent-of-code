use regex::Regex;
use std::collections::HashMap;
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
    top_edge: u32,
    width: u32,
    height: u32,
}

#[derive(Debug)]
struct BBox {
    xmin: u32,
    xmax: u32,
    ymin: u32,
    ymax: u32,
}

impl Claim {
    fn from_capture(c: regex::Captures) -> Self {
        Claim {
            id: c["id"].parse::<u32>().unwrap(),
            left_edge: c["left_edge"].parse::<u32>().unwrap(),
            top_edge: c["top_edge"].parse::<u32>().unwrap(),
            width: c["width"].parse::<u32>().unwrap(),
            height: c["height"].parse::<u32>().unwrap(),
        }
    }

    fn bounding_box(&self) -> BBox {
        BBox {
            xmin: self.left_edge,
            xmax: self.left_edge + self.width,
            ymin: self.top_edge + self.height,
            ymax: self.top_edge,
        }
    }
}

struct Locations(Vec<(u32, u32)>);

impl BBox {
    fn points(&self) -> Locations {
        let mut locations = Locations(Vec::new());
        for i in self.xmin..self.xmax {
            for j in self.ymin..self.ymax {
                locations.0.push((i, j))
            }
        }
        locations
    }
}

fn part1(input: &str) -> Result<(), Box<dyn std::error::Error>> {
    // brute force:
    // 1. scan through data to determine max coordinates in each dimension: O(k)
    // 2. double for loop over dimensions; for each iteration check if each id covers that and update a dict: O(k*n^2)
    // 3. iterate over dict and count entries where val is >=2
    //
    // Potential performance improvements: use an r-tree type structure
    //
    // By Claims:
    // - create a counts: HashMap<Location, u32> that counts claims in a location
    // - iterate through claims and for each point in the claim update the counts
    //
    // By Grid Chunks:
    // - divide into disjoint groups with a union find: cons: it is possible that they will all be in one group;
    // - divide grid into chunks and create a lookup of which id's have claims that overlap that chunk;
    // for each chunk use brute force. how to chunk?

    let mut claims: Vec<Claim> = vec![];
    let re = Regex::new(
        r"#(?<id>\d+) @ (?<left_edge>\d+),(?<top_edge>\d+): (?<width>\d+)x(?<height>\d+)",
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
    writeln!(io::stdout(), "{:#?}", &claims[..3])?;
    let max_height = claims.iter().fold(0, |acc, c| acc + c.height + c.top_edge);
    let max_width = claims.iter().fold(0, |acc, c| acc + c.width + c.left_edge);
    writeln!(
        io::stdout(),
        "max width: {}, max height: {}",
        max_width,
        max_height
    )?;

    let mut coverage = HashMap::<(u32, u32), u32>::new();
    // Brute Force
    // for i in 0..max_height {
    //     for j in 0..max_width {
    //         for claim in claims.iter() {
    //             if claim_covers_point(i, j, claim) {
    //                 coverage.entry((i, j)).and_modify(|c| *c += 1).or_insert(1);
    //             }
    //         }
    //     }
    // }

    // By Claim
    let mut bbox: BBox;
    let mut locations: Locations;
    for claim in claims.iter() {
        bbox = claim.bounding_box();
        locations = bbox.points();
        update_coverage(&mut coverage, locations);
    }
    for (k, v) in coverage.iter().take(3) {
        println!("{:?}, {v}", k);
    }
    let disputed = coverage.into_values().filter(|v| *v > 1).count();
    writeln!(io::stdout(), "{disputed}")?;
    Ok(())
}

fn update_coverage(coverage: &mut HashMap<(u32, u32), u32>, locations: Locations) {
    for (x, y) in locations.0.iter() {
        coverage
            .entry((*x, *y))
            .and_modify(|c| *c += 1)
            .or_insert(1);
    }
}

fn claim_covers_point(i: u32, j: u32, claim: &Claim) -> bool {
    (i >= claim.left_edge)
        & (i <= (claim.left_edge + claim.width))
        & (j >= claim.top_edge)
        & (j <= (claim.top_edge + claim.height))
}

fn part2(input: &str) -> std::io::Result<()> {
    Ok(())
}
