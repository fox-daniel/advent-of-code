use std::fs;
use std::io::Write;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let input = fs::read_to_string("input/input.txt")?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

fn part1(input: &str) -> Result<()> {
    // determine a max distance from distances between letters
    // determine boundary letters that will have infinite area
    // HashMap<Point, u8> where u8 is b'A' etc
    // the grid defines a graph
    // recursive graph search starting from each letter
    // for i in 0..n
    //   find points at distance n if they are not of a lower distance from another letter, handle ties
    //
    // memoize
    //
    let points: Vec<Point> = input.lines().map(Point::from).collect();
    println!("{:?}", &points[..3]);
    Ok(())
}

#[derive(Debug)]
struct Point {
    x: u32,
    y: u32,
}

impl From<&str> for Point {
    fn from(s: &str) -> Self {
        let coordinates: Vec<&str> = s.split(',').collect();
        Point {
            x: coordinates[0].trim().parse().expect("parse x as u32"),
            y: coordinates[1].trim().parse().expect("parse y as u32"),
        }
    }
}

fn part2(input: &str) -> Result<()> {
    Ok(())
}
