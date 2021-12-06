use std::fs;
use std::str::FromStr;

#[derive(Clone)]
struct LanternFish {
    timer: usize,
}

impl FromStr for LanternFish {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(LanternFish { timer: s.parse::<usize>().unwrap() })
    }
}

impl LanternFish {
    fn tick(&mut self) {
        self.timer -= 1;
    }

    fn spawns(&self) -> bool {
        self.timer == 0
    }

    fn reset(&mut self) {
        self.timer = 6;
    }
}

fn main() {
    let filename = String::from("sample.txt");
    let contents = fs::read_to_string(filename).expect("Error reading file!");
    println!("Sample: {}", contents);
    
    let fishes: Vec<LanternFish> = contents.trim().split(",").map(str::parse).map(Result::unwrap).collect();

    println!("Part 1: {}", part1(&fishes, 80));
    println!("Part 2: {}", part1(&fishes, 256));
}

fn part1(fishes: &[LanternFish], days: usize) -> usize {
    let mut school = fishes.to_vec();
    println!("School: {}", school.len());

    for day in 0..days {
        let mut new_school = Vec::new();
        let mut new_fish = 0;
        for mut fish in school {
            if fish.spawns() {
                new_fish += 1;         
                fish.reset();
            } else {
                fish.tick();
            }
            new_school.push(fish);
        }

        school = new_school;
        for _ in 0..new_fish {
            school.push(LanternFish {timer: 8});
        }
        println!("After {:?} day, new fish {:?}, school: {:?}", day + 1, new_fish, school.len());
    }
    school.len()
}

fn part2(fishes: &[LanternFish]) -> usize {
    0
}
