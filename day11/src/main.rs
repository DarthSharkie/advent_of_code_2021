use std::fs;
use std::collections::VecDeque;
use std::collections::HashSet;

type Map = Vec<Vec<u8>>;

trait Octopi {
    fn increment(&mut self);

    fn absorb(&mut self, x: usize, y: usize) -> Vec<(usize, usize)>;
    fn flash(&mut self) -> usize;

    fn reset(&mut self);
    fn print(&self);
}

impl Octopi for Map { 
    fn increment(&mut self) {
        for x in 0..self.len() {
            for y in 0..self[0].len() {
                self[x][y] += 1;
            }
        }
    }

    fn absorb(&mut self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut adjacent = Vec::new();
        for dx in 0..=2 {
            for dy in 0..=2 {
                if dx == 1 && dy == 1 { continue; }
                if x + dx > 0 && x + dx <= self.len() && y + dy > 0 && y + dy <= self[0].len() {
                    self[x+dx-1][y+dy-1] = u8::min(self[x+dx-1][y+dy-1] + 1u8, 10);
                    adjacent.push((x+dx-1, y+dy-1));
                }
            }
        }
        adjacent
    }

    fn flash(&mut self) -> usize {
        let mut to_flash = VecDeque::new();
        let mut flashed = HashSet::new();
        for x in 0..self.len() {
            for y in 0..self[0].len() {
                // For each adjacent (incl diagonal), increase energy levels, goto 2
                if self[x][y] > 9 {
                    to_flash.push_back((x, y));
                }
            }
        }
        while !to_flash.is_empty() {
            let (x, y) = to_flash.pop_front().unwrap();
            let adjacent: Vec<(usize, usize)> = self.absorb(x, y);
            adjacent
                .iter()
                .filter(|&(x, y)| !flashed.contains(&(*x, *y)) && self[*x][*y] == 10)
                .for_each(|&(x, y)| if !to_flash.contains(&(x, y)) { to_flash.push_back((x, y)) });
            flashed.insert((x, y));
        }
        flashed.len()
    }

    fn reset(&mut self) {
        for x in 0..self.len() {
            for y in 0..self[0].len() {
                if self[x][y] == 10 {
                    self[x][y] = 0;
                }
            }
        }
    }
    
    fn print(&self) {
        for x in 0..self.len() {
            for y in 0..self[0].len() {
                print!("{:x}", self[x][y]);
            }
            println!();
        }
        println!();
    }
}


fn main() {
    println!("Part 1: {}", part1(&mut parse("input11.txt"))); // 290691
    println!("Part 2: {}", part2(&mut parse("input11.txt"))); // 2768166558
}

fn parse(filename: &str) -> Map {
    let contents = fs::read_to_string(filename).expect("Error reading file!");
    contents.trim().lines().map(|s| {
        s.bytes().map(|b| b - b'0').collect()
    }).collect()
}

fn part1(map: &mut Map) -> usize {
    let mut flashes: usize = 0;
    for _step in 0..100 {
        // println!("Step {}", _step);
        // map.print();
        // 1. Increase energy levels
        map.increment();
        // 2. Flash all 9s
        flashes += map.flash();
        // 3. Reset all 9s to 0
        map.reset();
        // println!("Flashes: {}", flashes);
    }
    // map.print();
    flashes
}

fn part2(map: &mut Map) -> usize {
    let mut flashes: usize = 0;
    let mut step: usize = 0;
    while flashes != map.len() * map[0].len() {
        // println!("Step {}", step);
        // map.print();
        // 1. Increase energy levels
        map.increment();
        // 2. Flash all 9s
        flashes = map.flash();
        // 3. Reset all 9s to 0
        map.reset();
        // println!("Flashes: {}", flashes);
        step += 1;
    }
    // map.print();
    step
}

#[test]
fn test_part1() {
    assert_eq!(part1(&mut parse("sample.txt")), 1656);
}

#[test]
fn test_part2() {
    assert_eq!(part2(&mut parse("sample.txt")), 195);
}
