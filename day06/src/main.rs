use std::fs;

fn main() {
    let filename = String::from("input06.txt");
    let contents = fs::read_to_string(filename).expect("Error reading file!");

    let mut school: [usize; 9] = [0; 9];
    for timer in contents.trim().split(',').map(str::parse::<usize>).map(Result::unwrap) {
        school[timer] += 1;
    }

    println!("Part 1: {}", tick(&mut school.clone(), 80));  // 386536
    println!("Part 2: {}", tick(&mut school.clone(), 256)); // 1732821262171
}

fn tick(school: &mut [usize; 9], days: usize) -> usize {
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

