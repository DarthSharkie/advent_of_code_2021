use std::fs;
use std::time::Instant;
use std::collections::HashMap;
use std::collections::HashSet;

struct System {
    caves: HashMap<String, Cave>,
    paths: Vec<Path>,
}

impl System {
    fn new(s: &str) -> Self {
        let mut cave_set = HashSet::new();
        let mut paths = Vec::new();
        s.trim().lines().for_each(|line| {
            let (first, last) = line.split_once('-').unwrap();
            cave_set.insert(Cave::new(first.to_string()));
            cave_set.insert(Cave::new(last.to_string()));
        });
        let mut caves = HashMap::new();
        for cave in cave_set {
            caves.insert(cave.name.to_string(), cave.clone());
        }
        s.trim().lines().for_each(|line| {
            let (first, last) = line.split_once('-').unwrap();
            paths.push(Path { first: first.to_string(), last: last.to_string()});
        });
        System { caves, paths }
    }

    fn find_routes(&self, start: &str, end: &str) -> Vec<Vec<String>> {
        let mut routes: Vec<Vec<String>> = Vec::new();

        let mut route: Vec<(String, usize)> = Vec::new();
        let mut to_try: Vec<(String, usize)> = Vec::new();
        let mut depth: usize = 0;
        to_try.push((start.to_string(), depth));
        println!("Starting traversal");
        while let Some((cave, route_depth)) = to_try.pop() {
            //println!("Visiting {} at depth {}", cave, route_depth);
            //println!("Nodes to try: {:?}", to_try);
            while route_depth < depth {
                route.pop();
                depth -=1;
            }
            route.push((cave.to_string(), depth));

            if cave == end {
                // Copy route
                println!("Found a route: {:?}", route);
                routes.push(route.iter().map(|stop| stop.0.to_string()).collect());
            }
            for possible in self.select_possible_paths(&cave) {
                //println!("Could visit: {} -> {}", possible.first, possible.last);
                if possible.last == possible.last.to_ascii_lowercase() && 
                    route.iter().filter(|(name, _)| name == &possible.last).count() > 0 {
                    //println!("{} is a small cave, already been there!", possible.last);
                } else {
                    to_try.push((possible.last, depth + 1));
                }
            }
            depth += 1;
        }
        routes
    }

    fn select_possible_paths(&self, from: &str) -> Vec<Path> {
        self.paths.iter().filter(|p| p.first == from || p.last == from).map(|p| p.order(from)).collect()
    }
}

#[derive(Clone,Eq,Hash)]
struct Cave {
    name: String,
    big: bool,
    visited: bool,
}

impl Cave {
    fn new(name: String) -> Self {
        if name == name.to_ascii_lowercase() {
            Cave { name, big: false, visited: false }
        } else {
            Cave { name, big: true, visited: true }
        }
    }
}

impl PartialEq for Cave {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

#[derive(Eq,Hash)]
struct Path {
    first: String,
    last: String,
}

impl Path {
    fn order(&self, s: &str) -> Path {
        if s == self.first {
            Path {first: self.first.to_string(), last: self.last.to_string()}
        } else {
            Path {first: self.last.to_string(), last: self.first.to_string()}
        }
    }
}

impl PartialEq for Path {
    fn eq(&self, other: &Self) -> bool {
        (self.first == other.first && self.last == other.last) ||
            (self.first == other.last && self.last == other.first)
    }
}
    
fn main() {
    let start = Instant::now();
    println!("Part 1: {}", part1(&mut parse("input12.txt"))); // 290691
    println!("Part 2: {}", part2(&mut parse("input12.txt"))); // 2768166558
    let elapsed = start.elapsed();
    println!("Elapsed: {}µs", elapsed.as_micros());
}

fn parse<'a>(filename: &str) -> System {
    let contents = fs::read_to_string(filename).expect("Error reading file!");
    System::new(&contents)
}

fn part1(system: &System) -> usize {
    println!("Find routes from start to end!");
    let routes = system.find_routes("start", "end");
    routes.len()
}

fn part2(system: &System) -> usize {
    0
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

//#[test]
fn test_part2() {
    assert_eq!(part2(&mut parse("sample.txt")), 195);
}
