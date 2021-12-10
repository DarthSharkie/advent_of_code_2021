use std::fs;
use std::collections::VecDeque;
use std::collections::HashSet;

type Floor = Vec<Vec<u8>>;
type Point = (usize, usize);

fn main() {
    println!("Part 1: {}", part1(&parse("input09.txt"))); // 518
    println!("Part 2: {}", part2(&parse("input09.txt"))); // 949905
}

fn parse(filename: &str) -> Floor {
    let contents = fs::read_to_string(filename).expect("Error reading file!");
    contents.trim().lines().map(|s| {
        s.bytes().map(|b| b - b'0').collect::<Vec<u8>>()
    }).collect()
}

trait LessThan {
    fn less_than_all(&self, others: &Vec<u8>) -> bool;
}

impl LessThan for u8 {
    fn less_than_all(&self, others: &Vec<u8>) -> bool {
        others.iter().all(|o| self < o)
    }
}

fn part1(floor: &Floor) -> i32 {
    let low_points: Vec<(usize, usize)> = find_low_points(floor);

    low_points.iter().map(|(y,x)| (floor[*y][*x] + 1) as i32).sum()
}

fn part2(floor: &Floor) -> usize {
    let low_points: Vec<(usize, usize)> = find_low_points(floor);

    let mut basin_sizes: Vec<usize> = low_points.iter().map(|&p| find_basin(floor, p)).collect();
    basin_sizes.sort_unstable_by(|a,b| b.cmp(a));
    basin_sizes.iter().take(3).product()
}

fn find_basin(floor: &Floor, point: (usize, usize)) -> usize {
    let mut basin: HashSet<Point> = HashSet::new();
    let mut check: VecDeque<Point> = VecDeque::new();
    
    check.push_back(point);
    while !check.is_empty() {
        // Get point from queue
        let (y, x) = check.pop_front().unwrap();
        // If 9, not in a basin
        if floor[y][x] == 9 {
            continue;
        }
        // Put in basin, since it's not 9
        basin.insert((y,x));
        // Add four points in each cardinal direction if not already present and in floor
        if y >= 1 {
            if !basin.contains(&(y-1, x)) { 
                check.push_back((y-1, x));
            }
        }
        if x >= 1 {
            if !basin.contains(&(y, x-1)) { 
                check.push_back((y, x-1));
            }
        }
        if y < floor.len() - 1 {
            if !basin.contains(&(y+1, x)) { 
                check.push_back((y+1, x));
            }
        }
        if x < floor[0].len() - 1 {
            if !basin.contains(&(y, x+1)) { 
                check.push_back((y, x+1));
            }
        }
    }
    basin.len()
}

fn find_low_points(floor: &Floor) -> Vec<(usize, usize)> {
    let mut low_points = Vec::new();
    let height = floor.len();
    let width = floor[0].len();
    for y in 0..height {
        for x in 0..width {
            //println!("Point: {}, {} = {}", y, x, floor[y][x]);
            let low = if x == 0 {
                if y == 0 { 
                    // Top Left
                    floor[y][x].less_than_all(&vec![floor[y+1][x], floor[y][x+1]]) 
                } else if y == height - 1 { 
                    // Bottom Left
                    floor[y][x].less_than_all(&vec![floor[y-1][x], floor[y][x+1]])
                } else { 
                    // Left side
                    floor[y][x].less_than_all(&vec![floor[y+1][x], floor[y-1][x], floor[y][x+1]])
                }
            } else if x == width - 1 {
                if y == 0 { 
                    // Top Right
                    floor[y][x].less_than_all(&vec![floor[y+1][x], floor[y][x-1]]) 
                } else if y == height - 1 { 
                    // Bottom Right
                    floor[y][x].less_than_all(&vec![floor[y-1][x], floor[y][x-1]]) 
                } else { 
                    // Right side
                    floor[y][x].less_than_all(&vec![floor[y+1][x], floor[y-1][x], floor[y][x-1]]) 
                }
            } else {
                if y == 0 { 
                    // Top side
                    floor[y][x].less_than_all(&vec![floor[y+1][x], floor[y][x-1], floor[y][x+1]]) 
                } else if y == height - 1 { 
                    // Bottom side
                    floor[y][x].less_than_all(&vec![floor[y-1][x], floor[y][x-1], floor[y][x+1]]) 
                } else { 
                    // Middle
                    floor[y][x].less_than_all(&vec![floor[y-1][x], floor[y+1][x], floor[y][x-1], floor[y][x+1]]) 
                }
            };
            if low {
                low_points.push((y, x));
            }
        }
    }
    low_points
}


#[test]
fn test_part1() {
    assert_eq!(part1(&parse("sample.txt")), 15);
}

#[test]
fn test_part2() {
    assert_eq!(part2(&parse("sample.txt")), 1134);
}
