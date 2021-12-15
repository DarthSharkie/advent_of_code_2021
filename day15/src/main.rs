use std::fs;
use std::time::Instant;


type Map = Vec<Vec<usize>>;

#[derive(Copy,Clone,Debug,Eq)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn right(&self) -> Point {
        Point { x: self.x + 1, y: self.y }
    }

    fn down(&self) -> Point {
        Point { x: self.x, y: self.y + 1 }
    }

    fn up(&self) -> Point {
        Point { x: self.x, y: self.y - 1 }
    }

    fn left(&self) -> Point {
        Point { x: self.x - 1, y: self.y }
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        self.x == other.x && self.y == other.y
    }
}

fn main() {
    let start = Instant::now();
    println!("Part 1: {}", part1(&parse("input15.txt"))); // 2321
    println!("Part 2: {}", part2(&parse("input15.txt"))); // 118803
    let elapsed = start.elapsed();
    println!("Elapsed: {}µs", elapsed.as_micros());
}

fn parse(filename: &str) -> Map {
    let contents = fs::read_to_string(filename).expect("Error reading file!");
    contents.lines().map(|s| {
        s.bytes().map(|b| b - b'0').map(usize::from).collect()
    }).collect()
}

fn find_route(map: &Map, start: &Point, end: &Point) -> usize {
    // Use bool matrix as "set" membership, since I don't quite get BinaryHeap and Reverse, yet
    let mut marked: Vec<Vec<bool>> = vec![vec![false; map.len()]; map.len()];
    let mut distances: Vec<Vec<usize>> = vec![vec![usize::MAX; map.len()]; map.len()];
    let mut prev: Vec<Vec<Option<Point>>> = vec![vec![None; map.len()]; map.len()];
    let mut heap: Vec<Point> = Vec::new();

    // Starting point
    distances[0][0] = 0;
    heap.push(*start);

    while !heap.is_empty() {
        // Sort descending so we can pop off the end (should be faster than remove(0))
        // Given the frequency of distances and short edge weights, could Dial's algorithm help?
        heap.sort_unstable_by(|a, b| distances[b.y][b.x].cmp(&distances[a.y][a.x]));

        let nearest = heap.pop().unwrap();
        if nearest == *end {
            break;
        }
        marked[nearest.y][nearest.x] = true;

        let mut neighbors = Vec::new();
        if nearest.x < map.len() - 1 { neighbors.push(nearest.right()); }
        if nearest.y < map.len() - 1 { neighbors.push(nearest.down()); }
        if 0 < nearest.x { neighbors.push(nearest.left()); }
        if 0 < nearest.y { neighbors.push(nearest.up()); }

        for neighbor in neighbors {
            // Expand the frontier (a la 'uniform cost search', a Dijkstra optimization for large
            // graphs)
            if !marked[neighbor.y][neighbor.x] && !heap.contains(&neighbor) {
                heap.push(neighbor);
                let risk = distances[nearest.y][nearest.x] + map[neighbor.y][neighbor.x];
                if risk < distances[neighbor.y][neighbor.x] {
                    distances[neighbor.y][neighbor.x] = risk;
                    prev[neighbor.y][neighbor.x] = Some(nearest);
                }
            }
        }
    }
    distances[end.y][end.x]
}


fn part1(map: &Map) -> usize {
    find_route(map, &Point { x: 0, y: 0 }, &Point { x: map.len() - 1, y: map.len() - 1 })
}

fn part2(map: &Map) -> usize {
    let len = map.len();
    let mut big: Map = vec![vec![0; 5 * len]; 5 * len];
    for y in 0..map.len() {
        for x in 0..map.len() {
            for ry in 0..5 {
                for rx in 0..5 {
                    big[ry * len + y][rx * len + x] = map[y][x] + ry + rx;
                    if big[ry * len + y][rx * len + x] > 9 {
                        big[ry * len + y][rx * len + x] -= 9;
                    }
                }
            }
        }
    }
    find_route(&big, &Point { x: 0, y: 0 }, &Point { x: big.len() - 1, y: big.len() - 1 })
}


#[test]
fn test_part1a() {
    assert_eq!(part1(&parse("sample.txt")), 40);
}

#[test]
fn test_part2a() {
    assert_eq!(part2(&parse("sample.txt")), 315);
}
