use std::collections::{HashMap, HashSet};
use std::fs;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let input = fs::read_to_string("input/input.txt")?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

    /// Determine a bounding box and only process points inside it, including the boundary
    /// - infinite area points are identified as follows:
    ///   - for each boundary point, find the reference point for it; that reference point will have infinite area
    /// - Breadth first search starting from a reference points: for each distance, check all points that distance from a reference point.
    /// - update the status of each point in the search
    ///   - None
    ///   - Assigned(Assignment {reference, distance})
    ///   - Tied
    /// - filter out points with infinite area: if a point on the boudary references point P, then P will have infinite area.
    ///
fn part1(input: &str) -> Result<()> {
    let points: Vec<Point> = input.lines().map(Point::from).collect();
    let mut area_map = HashMap::<Point, usize>::new();
    let mut territory = HashMap::<Point, Status>::new();
    let bb = BoundingBox::from_points(&points);
    let max_distance = bb.max_distance_to_a_reference_point();
    println!("max distance: {max_distance}");
    for dist in 1..=max_distance {
        for ref_point in points.iter() {
            update_assignments(ref_point, dist, &mut territory);
        }
    }
    let boundary_points: HashSet<Point>  = bb.get_boundary_points().iter().cloned().collect();
    territory.iter().filter(|(p, _)| !boundary_points.contains(p)).for_each(|(_, v)| {
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

fn update_assignments(ref_point: &Point, dist: usize, territory: &mut HashMap<Point, Status>) {
    let points_at_a_distance = get_points_at_a_distance(ref_point, dist, territory);
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

fn is_on_outer_boundary(point: &Point, bb: &BoundingBox) -> bool {
    point.x == bb.xmin
    || point.x == bb.xmax
    || point.y == bb.ymin
    || point.y == bb.ymax
}
// would be better to use a generator here
fn get_points_at_a_distance(point: &Point, distance: usize, territory: &HashMap<Point, Status>) -> Vec<Point> {
    let mut points = Vec::new();
    let x = point.x;
    let y = point.y;
    for j in 0..distance {
        let dist = distance as i32;
        let p1 = Point {
            x: x - j as i32,
            y: y + dist - j as i32,
        };
        let p2 = Point {
            x: x - dist + j as i32,
            y: y - j as i32,
        };
        let p3 = Point {
            x: x + j as i32,
            y: y - dist + j as i32,
        };
        let p4 = Point {
            x: x + dist - j as i32,
            y: y + j as i32,
        };
        for p in [p1, p2, p3, p4] {
            if territory.get(&p).is_none() {
                points.push(p);
            }
            else if let Some(Status::Assigned(_)) = territory.get(&p) {
                points.push(p);
            }
        }
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

    fn height(&self) -> usize {
        (self.ymax - self.ymin) as usize
    }
    
    fn width(&self) -> usize {
        (self.xmax - self.xmin) as usize
    }

    fn get_boundary_points(&self) -> Vec<Point> {
        let mut points = Vec::new();
        for i in 0..=self.height() {
            points.push(
                Point {
                    x: self.xmin,
                    y: self.ymin + i as i32,
                }
            );
            points.push(
                Point {
                    x: self.xmax,
                    y: self.ymin + i as i32,
                }
            );
        }
        for i in 1..self.width() {
            points.push(
                Point {
                    x: self.xmin + i as i32,
                    y: self.ymin,
                }
            );
            points.push(
                Point {
                    x: self.xmin + i as i32,
                    y: self.ymax,
                }
            );
        }
        points
    }

    fn max_distance_to_a_reference_point(&self) -> usize {
        self.height()/2 + self.width()/2
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

    #[test]
    fn test_get_boundary_points() {
        let points = vec![Point {x:-1,y:-4}, Point {x:2, y:3}];
        let bb = BoundingBox::from_points(&points);
        let boundary_points = bb.get_boundary_points();
        let h = bb.height();
        let w = bb.width();
        let num_points = 2*(w+h);
        assert_eq!(boundary_points.len(),num_points);
    }
    
    #[test]
    fn test_max_boundary() {
        let points = vec![Point {x:0,y:0}, Point {x:4, y:4}];
        /*
        Example:
        r x x x x
        x x x x x
        x x p x x
        x x x x x
        x x x x r
        */
        let bb = BoundingBox::from_points(&points);
        let dist = bb.max_distance_to_a_reference_point();
        assert_eq!(dist, 4);
    }

    #[test]
    fn test_get_points_at_a_distance() {
        let territory = HashMap::new();
        let point = Point {x: 0, y: 0};
        let distance = 3;
        let points = get_points_at_a_distance(&point, distance, &territory);
        println!("{points:?}");
        assert_eq!(points.len(), 12);
    }
}
