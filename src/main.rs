use std::fs;

fn main() {
    let filename = String::from("/mnt/s/AdventOfCode/2021/day01/input01.txt");

    // println!("Input:\n{}", contents);
    //
    let lines: Vec<String> = load_file(&filename);
    
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

fn load_file(filename: &String) -> Vec<String> {
    let contents = fs::read_to_string(filename).expect("Error reading file!");
    let lines: Vec<String> = contents.trim().split('\n').map(|s| s.trim()).map(|s| String::from(s)).collect();
    lines
}
