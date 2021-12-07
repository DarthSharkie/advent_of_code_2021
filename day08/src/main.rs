use std::fs;

fn main() {
    println!("Part 1: {}", part1(parse("input07.txt"))); // 386536
    println!("Part 2: {}", part2(parse("input07.txt"))); // 1732821262171
}

fn parse(filename: &str) -> Vec<i32> {
    let contents = fs::read_to_string(filename).expect("Error reading file!");
    contents.trim().split(',').map(str::parse).map(Result::unwrap).collect()
}

fn part1(positions: Vec<i32>) -> i32 {
    // minimize_fuel(positions, |d| d)
    0
}

fn part2(positions: Vec<i32>) -> i32 {
    // minimize_fuel(positions, |d| d * (d+1) / 2)
    0
}

fn minimize_fuel(positions: Vec<i32>, fuel: fn(i32) -> i32) -> i32 {
    let min = positions.iter().min().unwrap();
    let max = positions.iter().max().unwrap();
    let mut min_fuel = i32::MAX;
    let mut best_position = 0;

    for pos in *min..=*max {
        let sum = positions.iter().map(|x| fuel((pos - x).abs())).sum();
        if sum < min_fuel {
            min_fuel = sum;
            best_position = pos;
        }
    }
    println!("Position {} is best", best_position);
    min_fuel
}


#[test]
fn test_part1() {
    assert_eq!(part1(parse("sample.txt")), 0);
}

#[test]
fn test_part2() {
    assert_eq!(part2(parse("sample.txt")), 0);
}
