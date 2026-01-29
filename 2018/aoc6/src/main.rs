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
    // determine a bounding box and only process points inside it:
    //   - this needs to extend w+l outside the minimal box, where w and l are the dimensions of the minimal bounding box
    //   - the reason for this is that points interior to the bounding box edges can still have infinite area
    // HashMap<Point, Status>
    // - Status indicates Unassigned, Assigned {reference, distance}, Tied
    //
    // - ASCII-CODE(A)=64
    // - ASCII-CODE(A)=97
    //
    // graph search starting from each letter
    // let n be the half width of the bounding box
    // for i in 0..2n
    //   - for each ref point
    //     - find points at distance n from
    //     - assign if not assigned
    //     - handle ties: if assigned is same distance, switch to tie sentinal
    //     - memoize
    //
    //
    //
    // x     x
    //    x  x
    //    x  x
    //       x
    //
    //
    //
    let points: Vec<Point> = input.lines().map(Point::from).collect();
    println!("{:?}", &points[..3]);
    let mut territory = HashMap::<Point, Status>::new();
    let mut bb = BoundingBox::new(points[0].x, points[0].y, points[0].x, points[0].y);
    bb = points.iter().fold(bb, |mut bb, point| {
        bb.xmin = i32::min(bb.xmin, point.x);
        bb.xmax = i32::max(bb.xmax, point.x);
        bb.ymin = i32::min(bb.ymin, point.y);
        bb.ymax = i32::max(bb.ymax, point.y);
        bb
    });
    let hwidth = (bb.xmax - bb.xmin) / 2 + 1;
    let hheight = (bb.ymax - bb.ymin) / 2 + 1;
    bb.xmin -= hwidth;
    bb.xmax += hwidth;
    bb.ymin -= hheight;
    bb.ymax += hheight;
    let max_distance = (bb.xmax - bb.xmin + bb.ymax - bb.ymin) / 2;
    for distance in 1..=max_distance {
        for point in points.iter() {
            let points_at_a_distance = get_points_at_a_distance(point, distance as usize);
        }
    }
    Ok(())
}

// would be better to use a generator here
fn get_points_at_a_distance(point: &Point, distance: usize) -> Vec<Point> {
    let mut points = Vec::with_capacity(4 * distance);
    let x = point.x;
    let y = point.y;
    for j in 0..distance {
        let dist = distance as i32;
        points.push(Point {
            x: x - j as i32,
            y: y + dist - j as i32,
        });
        points.push(Point {
            x: x - dist + j as i32,
            y: y - j as i32,
        });
        points.push(Point {
            x: x + j as i32,
            y: y - dist + j as i32,
        });
        points.push(Point {
            x: x + dist - j as i32,
            y: y + j as i32,
        });
    }
    points
}

#[derive(Debug, Clone)]
enum Status {
    Assigned,
    Tied,
}

#[derive(Debug, Clone)]
struct Assigned {
    reference: u8,
    distance: usize,
}

#[derive(Debug, Hash, std::cmp::Eq, std::cmp::PartialEq, Clone)]
struct Point {
    x: i32,
    y: i32,
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
struct BoundingBox {
    xmin: i32,
    ymin: i32,
    xmax: i32,
    ymax: i32,
}

impl BoundingBox {
    fn new(xmin: i32, ymin: i32, xmax: i32, ymax: i32) -> Self {
        BoundingBox {
            xmin,
            ymin,
            xmax,
            ymax,
        }
    }
}

// fn initialize_points<'a>(
//     bb: BoundingBox,
//     territory: &mut HashMap<Point, Status>,
//     point: Point,
// ) -> (BoundingBox, &'a mut HashMap<Point, Status>) {
//     bb.xmin = cmp::min(bb.xmin, Some(point.x));
//     bb.ymin = cmp::min(bb.ymin, Some(point.y));
//     bb.xmax = cmp::max(bb.xmax, Some(point.x));
//     bb.ymax = cmp::max(bb.ymax, Some(point.y));
//     territory.insert(point.clone(), Status::Unassigned);
//     (bb, stuff.1)
// }

fn part2(input: &str) -> Result<()> {
    Ok(())
}
