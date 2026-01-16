use std::collections::HashMap;
use std::fs;
// use std::io::Write;
use std::cmp;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let input = fs::read_to_string("input/input.txt")?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

fn part1(input: &str) -> Result<()> {
    // determine a bounding box and only process points inside it
    // HashMap<Point, (u8, Dist)> where u8 is, e.g. b'A', assigning the point to a reference point; Dist is enum with distance to ref point, unassigned, or tie
    // - use b'A' for ref point 'A' and 'a' if the point is assigned to 'A'
    // the grid defines a graph
    // graph search starting from each letter
    // for i in 0..n
    //   - for each ref point
    //     - find points at distance n from
    //     - assign if not assigned
    //     - handle ties: if assigned is same distance, switch to tie sentinal
    //     - memoize
    //
    // determine boundary letters that will have infinite area
    // - any ref point on the boundary of the convex hull will have inf area
    // - find convex hull boundary
    // - filter out those points
    //
    //
    // x     x
    //    x  x
    //    x  x
    //       x
    //
    // find max of remaining
    //
    //
    let points: Vec<Point> = input.lines().map(Point::from).collect();
    println!("{:?}", &points[..3]);
    let mut territory = HashMap::<Point, Status>::new();
    let bb = BoundingBox::new();
    for point in points.into_iter() {
        bb = initialize_points(bb, territory, point);
    }

    Ok(())
}

#[derive(Debug, Hash, std::cmp::Eq, std::cmp::PartialEq, Clone)]
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

#[derive(Debug)]
enum Status {
    Distance(u32),
    Unassigned,
    Tie,
}

#[derive(Debug)]
struct BoundingBox {
    xmin: Option<u32>,
    ymin: Option<u32>,
    xmax: Option<u32>,
    ymax: Option<u32>,
}

impl BoundingBox {
    fn new() -> Self {
        BoundingBox {
            xmin: None,
            ymin: None,
            xmax: None,
            ymax: None,
        }
    }
}

fn initialize_points<'a>(
    bb: BoundingBox,
    territory: &mut HashMap<Point, Status>,
    point: Point,
) -> (BoundingBox, &'a mut HashMap<Point, Status>) {
    bb.xmin = cmp::min(bb.xmin, Some(point.x));
    bb.ymin = cmp::min(bb.ymin, Some(point.y));
    bb.xmax = cmp::max(bb.xmax, Some(point.x));
    bb.ymax = cmp::max(bb.ymax, Some(point.y));
    territory.insert(point.clone(), Status::Unassigned);
    (bb, stuff.1)
}

fn part2(input: &str) -> Result<()> {
    Ok(())
}
