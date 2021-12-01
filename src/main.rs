use std::fs;

fn main() {
    let filename = String::from("/mnt/s/AdventOfCode/2021/day01/input01.txt");

    // println!("Input:\n{}", contents);
    //
    let lines: Vec<String> = load_file(&filename);
    
    part1(&lines);
    part2(&lines);
}

fn load_file(filename: &String) -> Vec<String> {
    let contents = fs::read_to_string(filename).expect("Error reading file!");
    let lines: Vec<String> = contents.trim().split('\n').map(|s| s.trim()).map(|s| String::from(s)).collect();
    lines
}

fn part1(lines: &Vec<String>) {
    let mut previous_depth: i32 = i32::MAX;
    let mut descents: i32 = 0;

    for line in lines {
        println!("Depth: {}", line);
        let depth: i32 = line.parse().expect("Invalid i32 value");
        if depth > previous_depth {
            println!("Descending!");
            descents += 1;
        }
        previous_depth = depth;
    }

    println!("Descended {} times!", descents);
    
}

fn part2(lines: &Vec<String>) {

    // Rolling index into the vector (really, a ring buffer)
    let mut index: usize = 0;
    let mut depths = vec![0; 3];

    // Depth tracking and descent counting
    let mut previous_depth: i32 = i32::MAX;
    let mut descents: i32 = 0;

    for line in lines {
        let depth: i32 = line.parse().expect("Invalid i32 value");
        depths[index.rem_euclid(3)] = depth;

        // If three values exist, then look for deeper water
        if index >= 2 {
            let depth_sum = depths[0] + depths[1] + depths[2];
            if depth_sum > previous_depth {
                descents += 1;
                println!("Descending!");
            }
            previous_depth = depth_sum;
        }
        index += 1;
    }

    println!("Descended {} times!", descents);
}
