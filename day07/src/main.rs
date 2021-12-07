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
    minimize_fuel(positions, |pos, x| i32::abs(pos - x))
}

fn part2(positions: Vec<i32>) -> i32 {
    minimize_fuel(positions, |pos, x| { let d = i32::abs(pos - x); d * (d+1) / 2})
}

fn minimize_fuel(positions: Vec<i32>, func: fn(i32, i32) -> i32) -> i32 {
    let min = positions.iter().min().unwrap();
    let max = positions.iter().max().unwrap();
    let mut fuel = i32::MAX;
    let mut best_position = 0;

    for pos in *min..=*max {
        let sum = positions.iter().fold(0, |acc, x| acc + func(pos, *x));
        if sum < fuel {
            fuel = sum;
            best_position = pos;
        }
    }
    println!("Position {} is best", best_position);
    fuel
}


#[test]
fn test_part1() {
    assert_eq!(part1(parse("sample.txt")), 37);
}

#[test]
fn test_part2() {
    assert_eq!(part2(parse("sample.txt")), 168);
}