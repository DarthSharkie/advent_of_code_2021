use std::fs;
use std::time::Instant;
use std::collections::HashMap;

struct System {
    paths: HashMap<String, Vec<String>>,
}

impl System {
    fn new(s: &str) -> Self {
        let mut paths: HashMap<String, Vec<String>> = HashMap::new();
        s.trim().lines().for_each(|line| {
            let (first, last) = line.split_once('-').unwrap();
            let first_neighbors = paths.entry(first.to_string()).or_insert(Vec::new());
            first_neighbors.push(last.to_string());
            let last_neighbors = paths.entry(last.to_string()).or_insert(Vec::new());
            last_neighbors.push(first.to_string());
        });
        println!("Paths: {:?}", paths);
        System { paths }
    }

    fn find_routes(&self, start: &str, end: &str, can_visit: fn(&HashMap<&str, usize>, &str) -> bool) -> usize {
        let mut routes = 0;
        let mut route: Vec<&str> = Vec::new();
        let mut stop_counts: HashMap<&str, usize> = HashMap::new();
        let mut to_try: Vec<(&str, usize)> = Vec::new();
        let mut depth: usize = 0;

        to_try.push((start, depth));
        while let Some((cave, route_depth)) = to_try.pop() {
            while route_depth < route.len() {
                let old_cave = route.pop().unwrap();
                if let Some('a'..='z') = old_cave.chars().next() {
                    *stop_counts.entry(old_cave).or_insert(1) -= 1
                }
                depth -=1;
            }
            route.push(cave);
            if let Some('a'..='z') = cave.chars().next() {
                *stop_counts.entry(cave).or_default() += 1
            }

            if cave == end {
                routes += 1;
            } else {
                for possible in self.paths.get(cave).unwrap() {
                    match &possible[..] {
                        "start" => (),
                        "end" => to_try.push((possible, depth + 1)),
                        _ => match possible.chars().next() {
                            Some('A'..='Z') => to_try.push((possible, depth + 1)),
                            Some('a'..='z') => if can_visit(&stop_counts, &possible) {
                                to_try.push((possible, depth + 1))
                            },
                            _ => panic!("Not a valid cave!"),
                        },
                    }
                }
            }
            depth += 1;
        }
        routes
    }
}

fn main() {
    let start = Instant::now();
    println!("Part 1: {}", part1(&mut parse("input12.txt"))); // 4413
    println!("Part 2: {}", part2(&mut parse("input12.txt"))); // 118803
    let elapsed = start.elapsed();
    println!("Elapsed: {}µs", elapsed.as_micros());
}

fn parse(filename: &str) -> System {
    let contents = fs::read_to_string(filename).expect("Error reading file!");
    System::new(&contents)
}

fn part1(system: &System) -> usize {
    system.find_routes("start", "end", |stop_counts, stop| { 
        stop_counts.get(stop).unwrap_or(&0) == &0 
    })
}

fn part2(system: &System) -> usize {
    system.find_routes("start", "end", |stop_counts, stop| { 
        stop_counts.get(stop).unwrap_or(&0) == &0 ||
            stop_counts.values().filter(|&count| count >= &2).count() == 0
    })
}


#[test]
fn test_part1a() {
    assert_eq!(part1(&mut parse("sample1.txt")), 10);
}

#[test]
fn test_part1b() {
    assert_eq!(part1(&mut parse("sample2.txt")), 19);
}

#[test]
fn test_part1c() {
    assert_eq!(part1(&mut parse("sample3.txt")), 226);
}

#[test]
fn test_part2a() {
    assert_eq!(part2(&mut parse("sample1.txt")), 36);
}

#[test]
fn test_part2b() {
    assert_eq!(part2(&mut parse("sample2.txt")), 103);
}

#[test]
fn test_part2c() {
    assert_eq!(part2(&mut parse("sample3.txt")), 3509);
}

