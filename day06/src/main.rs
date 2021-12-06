use std::fs;

fn main() {
    println!("Part 1: {}", part1(parse("input06.txt"))); // 386536
    println!("Part 2: {}", part2(parse("input06.txt"))); // 1732821262171
}

fn parse(filename: &str) -> [usize; 9] {
    let contents = fs::read_to_string(filename).expect("Error reading file!");
    let mut school: [usize; 9] = [0; 9];
    for timer in contents.trim().split(',').map(str::parse::<usize>).map(Result::unwrap) {
        school[timer] += 1;
    }
    school
}

fn part1(school: [usize; 9]) -> usize {
    tick(school, 80)
}

fn part2(school: [usize; 9]) -> usize {
    tick(school, 256)
}

fn tick(school: [usize; 9], days: usize) -> usize {
    let mut school = school.clone();
    for _ in 0..days {
        let new_fish = school[0];
        for timer in 0..8 {
            school[timer] = school[timer + 1];
        }
        school[6] += new_fish;
        school[8] = new_fish;
    }
    school.iter().sum()
}

#[test]
fn test_part1() {
    assert_eq!(part1(parse("sample.txt")), 5934);
}

#[test]
fn test_part2() {
    assert_eq!(part2(parse("sample.txt")), 26984457539);
}
