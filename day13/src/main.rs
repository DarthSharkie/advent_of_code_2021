use std::fs;
use std::time::Instant;
use std::collections::HashSet;

enum Fold {
    Horizontal(usize),
    Vertical(usize),
}

impl Fold {
    fn fold(&self, points: &mut HashSet<(usize, usize)>) {

        let mut new_points = HashSet::new();

        for point in points.iter() {
            let (x, y) = point;
            let new_point = match self {
                Fold::Horizontal(axis) => if y > axis { (*x, 2*axis - y) } else { (*x, *y) },
                Fold::Vertical(axis) => if x > axis { (2*axis - x, *y) } else { (*x, *y) },
            };
            println!("Old: {:?}, New: {:?}", point, new_point);
            new_points.insert(new_point);
        }
        // Have to insert here to avoid a mutable use of points
        for new_point in new_points.iter() {
            points.insert(*new_point);
        }
        println!("New Points: {:?}", new_points);
        points.retain(|point| new_points.contains(point));
    }
}


fn main() {
    let start = Instant::now();
    println!("Part 1: {}", part1(&parse("input13.txt"))); // 4413
    println!("Part 2: {}", part2(&parse("input13.txt"))); // 118803
    let elapsed = start.elapsed();
    println!("Elapsed: {}µs", elapsed.as_micros());
}

fn parse(filename: &str) -> (HashSet<(usize, usize)>, Vec<Fold>) {
    let contents = fs::read_to_string(filename).expect("Error reading file!");
    let mut content_iter = contents.split("\n\n");
    let mut point_set: HashSet<(usize, usize)> = HashSet::new();
    content_iter.next().unwrap().lines().for_each(|s| {
        let (x, y) = s.split_once(',').unwrap();
        point_set.insert((x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap()));
    });
    let folds = content_iter.next().unwrap().lines().map(|s| {
        let (axis, position) = s.split_whitespace().nth(2).unwrap().split_once('=').unwrap();
        let position = position.parse::<usize>().unwrap();

        match axis {
            "x" => Fold::Vertical(position),
            "y" => Fold::Horizontal(position),
            _ => panic!("Bad axis!"),
        }
    }).collect();
    (point_set, folds)
}

fn part1(system: &(HashSet<(usize, usize)>, Vec<Fold>)) -> usize {
    let (orig_points, folds) = system;
    let mut points = orig_points.clone();
    println!("Points: {:?}", points);
    for fold in folds.iter().take(1) {
        match fold {
            Fold::Horizontal(axis) => println!("Horizontal at y={}", axis),
            Fold::Vertical(axis) => println!("Vertical at x={}", axis),
        };
        fold.fold(&mut points);
        println!("Points: {:?}", points);
    }
    points.len()
}

fn part2(system: &(HashSet<(usize, usize)>, Vec<Fold>)) -> usize {
    let (orig_points, folds) = system;
    let mut points = orig_points.clone();
    println!("Points: {:?}", points);
    for fold in folds {
        match fold {
            Fold::Horizontal(axis) => println!("Horizontal at y={}", axis),
            Fold::Vertical(axis) => println!("Vertical at x={}", axis),
        };
        fold.fold(&mut points);
        println!("Points: {:?}", points);
    }

    let (mx, my) = points.iter().fold((0,0), |(mx, my), &(x, y)| (usize::max(mx, x), usize::max(my, y)));
    for y in 0..=my {
        for x in 0..=mx {
            if points.contains(&(x, y)) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }
    points.len()
}


#[test]
fn test_part1a() {
    assert_eq!(part1(&parse("sample.txt")), 17);
}

#[test]
fn test_part2a() {
    assert_eq!(part2(&parse("sample.txt")), 16);
}
