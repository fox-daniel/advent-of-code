use std::collections::{HashMap, HashSet};
use std::fs;

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
    //
    // need to filter out points with infinite area
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
    let mut area_map = HashMap::<Point, usize>::new();
    let mut territory = HashMap::<Point, Status>::new();
    let mut bb = BoundingBox::from_points(&points);
    bb.expand();
    let max_distance = bb.max_distance();
    println!("max distance: {max_distance}");
    for dist in 1..=max_distance {
        for ref_point in points.iter() {
            let points_at_a_distance = get_points_at_a_distance(ref_point, dist);
            // println!("num points at a distance: {}", points_at_a_distance.len());
            for point in points_at_a_distance.iter() {
                territory
                    .entry(point.clone())
                    .and_modify(|e| {
                        if let Status::Assigned(Assignment {
                            distance: distance_to_ref,
                            ..
                        }) = e && dist == *distance_to_ref {
                                *e = Status::Tied;
                        }
                    })
                    .or_insert(Status::Assigned(Assignment {
                        reference: ref_point.clone(),
                        distance: dist,
                    }));
            }
        }
    }
    let on_boundary = |point: &Point| is_on_outer_boundary(point, &bb);
    let infinite_area_points: HashSet<Point> = territory.keys().cloned().filter(on_boundary).collect();
    territory.iter().filter(|(p, _)| !infinite_area_points.contains(p)).for_each(|(_, v)| {
        if let Status::Assigned(Assignment {
            reference,
            ..
        }) = v {
            area_map.entry(reference.clone()).and_modify(|count| *count += 1).or_insert(1);
        }
    });
    println!("{area_map:?}");
    let max_area = area_map.iter().max_by_key(|item| item.1);
    println!("part1: {max_area:?}"); 
    Ok(())
}


fn is_on_outer_boundary(point: &Point, bb: &BoundingBox) -> bool {
    point.x == bb.xmin
    || point.x == bb.xmax
    || point.y == bb.ymin
    || point.y == bb.ymax
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

#[derive(Debug, Clone, PartialEq)]
enum Status {
    Assigned(Assignment),
    Tied,
}

#[derive(Debug, Clone, PartialEq)]
struct Assignment {
    reference: Point,
    distance: usize,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
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

    fn from_points(points: &[Point]) -> Self {
        let mut bb = BoundingBox::new(points[0].x, points[0].y, points[0].x, points[0].y);
        bb = points.iter().fold(bb, |mut bb, point| {
            bb.xmin = i32::min(bb.xmin, point.x);
            bb.xmax = i32::max(bb.xmax, point.x);
            bb.ymin = i32::min(bb.ymin, point.y);
            bb.ymax = i32::max(bb.ymax, point.y);
            bb
        });
        bb
    }

    fn expand(&mut self) {
        let hwidth = (self.xmax - self.xmin) / 2 + 1;
        let hheight = (self.ymax - self.ymin) / 2 + 1;
        self.xmin -= hwidth;
        self.xmax += hwidth;
        self.ymin -= hheight;
        self.ymax += hheight;
    }

    fn max_distance(&self) -> usize {
        ((self.xmax - self.xmin + self.ymax - self.ymin) / 2) as usize
    }
}



fn part2(input: &str) -> Result<()> {
    println!("{}", input.len());
    Ok(())
}


#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_bb_construction_2_points() {
        let points = vec![Point {x:1,y:1}, Point {x:-1, y:0}];
        let bb = BoundingBox::from_points(&points);
        assert_eq!(bb.xmin, -1);        
        assert_eq!(bb.ymin, 0);        
        assert_eq!(bb.xmax, 1);        
        assert_eq!(bb.ymax, 1);        
    }
    
    #[test]
    fn test_bb_construction_4_points() {
        let points = vec![Point {x:3,y:5}, Point {x:-3, y:0}, Point {x:0, y:0}, Point {x:2, y:-4}];
        let bb = BoundingBox::from_points(&points);
        assert_eq!(bb.xmin, -3);        
        assert_eq!(bb.ymin,-4);        
        assert_eq!(bb.xmax, 3);        
        assert_eq!(bb.ymax, 5);        
    }
}
