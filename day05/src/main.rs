use std::fs;
use std::str::FromStr;
use std::collections::HashMap;

#[derive(Debug)]
struct LineSegment {
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
}

enum Orientation {
    VERTICAL(bool),
    HORIZONTAL(bool),
    DIAGONAL(bool),
}

impl FromStr for LineSegment {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(" -> ").unwrap();
        let (x1, y1) = x.split_once(",").unwrap();
        let (x2, y2) = y.split_once(",").unwrap();
        Ok(LineSegment { 
            x1: x1.parse().unwrap(), 
            y1: y1.parse().unwrap(), 
            x2: x2.parse().unwrap(), 
            y2: y2.parse().unwrap(), 
        })
    }
}

impl LineSegment {
    fn orientation(&self) -> Orientation {
        if self.x1 == self.x2 {
            Orientation::VERTICAL(true)
        } else if self.y1 == self.y2 {
            Orientation::HORIZONTAL(true)
        } else {
            Orientation::DIAGONAL(false)
        }
    }
}


fn main() {
    let filename = String::from("input05.txt");
    let contents = fs::read_to_string(filename).expect("Error reading file!");
    
    let segments: Vec<LineSegment> = contents.lines().map(str::parse).map(Result::unwrap).collect();

    println!("Part 1: {}", part1(&segments));
    println!("Part 2: {}", part2(&segments));
}

fn part1(segments: &[LineSegment]) -> i32 {
    create_map(&segments, false)
}

fn part2(segments: &[LineSegment]) -> i32 {
    create_map(&segments, true)
}

fn create_map(segments: &[LineSegment], use_diag: bool) -> i32 {
    let mut points = HashMap::new();
    for segment in segments {
        match segment.orientation() {
            Orientation::VERTICAL(_) => {
                let y_range = if segment.y1 < segment.y2 { segment.y1..=segment.y2 } else { segment.y2..=segment.y1 };
                for y in y_range {
                    let point = (segment.x1, y);
                    points.insert(point, points.get(&point).unwrap_or(&0) + 1);
                }
            },
            Orientation::HORIZONTAL(_) => {
                let x_range = if segment.x1 < segment.x2 { segment.x1..=segment.x2 } else { segment.x2..=segment.x1 };
                for x in x_range {
                    let point = (x, segment.y1);
                    points.insert(point, points.get(&point).unwrap_or(&0) + 1);
                }
            },
            Orientation::DIAGONAL(_) => {
                if use_diag {
                    println!("Segment: {:?}", segment);
                    let x_range: Box<dyn Iterator<Item=usize>> = if segment.x1 < segment.x2 { Box::new(segment.x1..=segment.x2) } else { Box::new((segment.x2..=segment.x1).rev()) };
                    let y_range: Box<dyn Iterator<Item=usize>> = if segment.y1 < segment.y2 { Box::new(segment.y1..=segment.y2) } else { Box::new((segment.y2..=segment.y1).rev()) };
                
                    for point in x_range.zip(y_range) {
                        println!("Point: {:?}", point);
                        points.insert(point, points.get(&point).unwrap_or(&0) + 1);
                    }
                }
            }
        }
    }
    points.values().filter(|&v| *v > 1).count() as i32
}
