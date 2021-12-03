use std::fs;
use std::str::FromStr;

// Replace with day-specific struct/enums
/*
struct Move {
    direction: String,
    distance: i32,
}

impl FromStr for Move {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data: Vec<&str> = s.trim().split(' ').collect();
        // Add day-specific struct building

    }
}
*/

fn main() {
    let filename = String::from("input03.txt");
    let contents = fs::read_to_string(filename).expect("Error reading file!");

    let readings: Vec<String> = contents.trim().lines().map(str::trim).map(String::from).collect();

    part1(&readings);
    part2(&readings);
}

fn part1(readings: &[String]) {
    let mut gamma: [usize; 12] = [0; 12];

    for reading in readings {
        let mut i: usize = 0;
        for bit in reading.bytes() {
            if bit == b'1' {
                gamma[i] += 1;
            }
            i += 1;
        }
    }
    gamma = gamma.map(|count| count >= readings.len() / 2).map(|b| b as usize);
    let gamma_value: u32 = bits_to_u32(&gamma);
    let epsilon_value: u32 = 2u32.pow(12) - gamma_value - 1;
    let power = gamma_value * epsilon_value;
    println!("Power: {}", power);
}

fn part2(readings: &[String]) {
}

fn bits_to_u32(bits: &[usize]) -> u32 {
    bits.iter().fold(0, |acc, b| acc * 2 + b) as u32
}

