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

const MSB: u32 = 11;

fn main() {
    let filename = String::from("input03.txt");
    let contents = fs::read_to_string(filename).expect("Error reading file!");

    let readings: Vec<u32> = contents.trim().lines().map(str::trim).map(|s| str_to_u32(s)).collect();

    part1(&readings);
    part2(&readings);
}

fn part1(readings: &[u32]) {
    let mut gamma: [u32; 12] = [0; 12];

    for reading in readings {
        for bit in (0..=MSB).rev() {
            // If the bit is set
            if reading | 2u32.pow(bit) == *reading {
                gamma[(MSB-bit) as usize] += 1;
            }
        }
    }
    println!("Gamma: {:?}", gamma);
    gamma = gamma.map(|count| count > readings.len() as u32 / 2).map(|b| b as u32);
    let gamma_value: u32 = gamma.iter().fold(0, |acc, b| acc * 2 + b);
    let epsilon_value: u32 = 2u32.pow(12) - gamma_value - 1;
    let power = gamma_value * epsilon_value;
    println!("Power: {}", power);
}

fn part2(readings: &[u32]) {
}

fn str_to_u32(bits: &str) -> u32 {
    bits.bytes().fold(0, |acc, b| acc * 2 + (b - b'0') as u32)
}

