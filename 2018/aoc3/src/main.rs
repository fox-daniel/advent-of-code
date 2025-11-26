use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::io;
use std::io::Write;
use std::str::FromStr;
use std::sync::LazyLock;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string("input/input.txt")?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}
/// Vertical distances are measured downward, so the top edge has a lower value than the bottom edge
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
    id: u32,
    xmin: u32,
    xmax: u32,
    ymin: u32,
    ymax: u32,
}

struct ClaimParseError(String);

impl FromStr for Claim {
    type Err = ClaimParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re: LazyLock<Regex> = LazyLock::new(|| {
            Regex::new(
                r"#(?<id>\d+) @ (?<left_edge>\d+),(?<top_edge>\d+): (?<width>\d+)x(?<height>\d+)",
            )
            .expect("Regex compiles")
        });
        let cap = re
            .captures(s)
            .ok_or(ClaimParseError("claim syntax error".to_string()))?;
        Claim::from_capture(cap).map_err(|e| ClaimParseError(e.to_string()))
    }
}

impl Claim {
    fn from_capture(c: regex::Captures) -> Result<Self, std::num::ParseIntError> {
        Ok(Claim {
            id: c["id"].parse()?,
            left_edge: c["left_edge"].parse()?,
            top_edge: c["top_edge"].parse()?,
            width: c["width"].parse()?,
            height: c["height"].parse()?,
        })
    }

    fn bounding_box(&self) -> BBox {
        BBox {
            id: self.id,
            xmin: self.left_edge,
            xmax: self.left_edge + self.width - 1,
            ymin: self.top_edge,
            ymax: self.top_edge + self.height - 1,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct Loc {
    x: u32,
    y: u32,
}

struct Place {
    loc: Loc,
    id: u32,
}

struct Locations(Vec<Loc>);
struct Places(Vec<Place>);

impl BBox {
    fn locations(&self) -> Locations {
        let mut locations = Locations(Vec::new());
        for i in self.xmin..=self.xmax {
            for j in self.ymin..=self.ymax {
                locations.0.push(Loc { x: i, y: j })
            }
        }
        locations
    }

    fn places(&self) -> Places {
        let mut places = Places(Vec::new());
        for i in self.xmin..=self.xmax {
            for j in self.ymin..=self.ymax {
                places.0.push(Place {
                    loc: Loc { x: i, y: j },
                    id: self.id,
                })
            }
        }
        places
    }
}
fn part1(input: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut claims: Vec<Claim> = vec![];

    for line in input.lines() {
        if let Ok(claim) = line.parse() {
            claims.push(claim)
        }
    }

    let mut coverage = HashMap::<Loc, u32>::new();

    let mut bbox: BBox;
    let mut locations: Locations;
    for claim in claims.iter() {
        bbox = claim.bounding_box();
        locations = bbox.locations();
        update_coverage(&mut coverage, locations);
    }
    let disputed = coverage.into_values().filter(|v| *v > 1).count();
    writeln!(io::stdout(), "{disputed}")?;
    Ok(())
}

fn update_coverage(coverage: &mut HashMap<Loc, u32>, locations: Locations) {
    for loc in locations.0.into_iter() {
        coverage
            .entry(loc.clone())
            .and_modify(|c| *c += 1)
            .or_insert(1);
    }
}

fn update_coverage_for_places(coverage: &mut HashMap<Loc, Vec<u32>>, places: Places) {
    for place in places.0.into_iter() {
        coverage
            .entry(place.loc.clone())
            .and_modify(|v| v.push(place.id))
            .or_insert(vec![place.id]);
    }
}
fn part2(input: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut claims: Vec<Claim> = vec![];

    for line in input.lines() {
        if let Ok(claim) = line.parse() {
            claims.push(claim)
        }
    }

    let mut coverage = HashMap::<Loc, Vec<u32>>::new();

    let mut bbox: BBox;
    let mut places: Places;
    for claim in claims.iter() {
        bbox = claim.bounding_box();
        places = bbox.places();
        update_coverage_for_places(&mut coverage, places);
    }
    let disputed: HashSet<u32> = coverage
        .clone()
        .into_iter()
        .filter(|item| item.1.len() > 1)
        .flat_map(|item| item.1.into_iter())
        .collect();
    let ids: HashSet<u32> = claims.into_iter().map(|c| c.id).collect();
    let undisputed_claim = ids
        .difference(&disputed)
        .next()
        .expect("There is a unique undisputed claim.");
    writeln!(io::stdout(), "undisputed: {:#?}", undisputed_claim).ok();
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    fn make_claims() -> Vec<Claim> {
        vec![
            Claim {
                id: 0,
                left_edge: 2,
                top_edge: 2,
                width: 2,
                height: 3,
            },
            // o o o o
            // o o o o
            // o o x x
            // o o x x
            // o o x x
            Claim {
                id: 1,
                left_edge: 3,
                top_edge: 1,
                width: 2,
                height: 2,
            },
            // o o o o o
            // o o o x x
            // o o o x x
        ]
    }

    #[test]
    fn locations_from_bounding_box() {
        let bbox = BBox {
            id: 1234,
            xmin: 1,
            xmax: 2,
            ymin: 3,
            ymax: 4,
        };
        let locations = bbox.locations();
        let loc = &locations.0[0];
        assert_eq!(&Loc { x: 1, y: 3 }, loc);
        assert_eq!(4, locations.0.len());
    }

    #[test]
    fn bbox_from_claim() {
        let claims = make_claims();
        let bbox = claims[0].bounding_box();
        assert_eq!(2, bbox.xmin);
        assert_eq!(3, bbox.xmax);
        assert_eq!(2, bbox.ymin);
        assert_eq!(4, bbox.ymax);
    }

    #[test]
    fn update_coverage_test() {
        let claims = make_claims();
        let mut coverage = HashMap::<Loc, u32>::new();
        for claim in claims.iter() {
            let locations = claim.bounding_box().locations();
            update_coverage(&mut coverage, locations);
        }
        assert_eq!(2, *coverage.get(&Loc { x: 3, y: 2 }).unwrap())
    }
}
