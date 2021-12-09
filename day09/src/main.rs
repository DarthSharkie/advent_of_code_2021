use std::fs;
use std::cmp::min;

type Floor = Vec<Vec<u8>>;

fn main() {
    println!("Part 1: {}", part1(&parse("input09.txt"))); // 543
    println!("Part 2: {}", part2(&parse("input09.txt"))); // 994266
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
        println!("others: {:?}", others);
        others.iter().all(|o| self < o)
    }
}

fn part1(floor: &Floor) -> i32 {
    let mut low_points: Vec<i32> = Vec::new();
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
                low_points.push((floor[y][x] + 1) as i32);
            }
        }
    }
    low_points.iter().sum()
}

fn part2(_floor: &Floor) -> i32 {
    0
}


#[test]
fn test_part1() {
    assert_eq!(part1(&parse("sample.txt")), 15);
}

#[test]
fn test_part2() {
    assert_eq!(part2(&parse("sample.txt")), 61229);
}
