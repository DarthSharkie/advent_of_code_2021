use std::fs;

fn main() {
    let filename = String::from("/mnt/s/AdventOfCode/2021/day01/input01.txt");
    let lines: Vec<u32> = load_file(&filename);
    
    part1(&lines);
    part2(&lines);
}

fn load_file(filename: &str) -> Vec<u32> {
    let contents = fs::read_to_string(filename).expect("Error reading file!");
    let lines: Vec<u32> = contents.trim().lines().map(|s| s.parse().expect("Invalid u32 value")).collect();
    lines
}

fn part1(depths: &[u32]) {
    let mut previous_depth: u32 = u32::MAX;
    let mut descents: u32 = 0;

    for depth in depths {
        if *depth > previous_depth {
            descents += 1;
        }
        previous_depth = *depth;
    }

    println!("Descended {} times!", descents);
}

fn part2(depths: &[u32]) {
    let mut descents: u32 = 0;

    for (index, depth) in depths.into_iter().enumerate() {
        // If three values exist, then look for deeper water
        if index >= 3 {
            if *depth > depths[index - 3] {
                descents += 1;
            }
        }
    }

    println!("Descended {} times!", descents);
}
