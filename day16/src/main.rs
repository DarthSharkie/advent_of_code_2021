use std::fs;
use std::time::Instant;


struct Transmission {
}

struct Packet {
    version: u8,
    type_id: u8,
    literal: Option<usize>,
    subpackets: Vec<Packet>,
}

impl Packet {
    fn version_sum(&self) -> usize {
        self.subpackets.iter().map(Self::version_sum).sum::<usize>() + self.version as usize
    }
}


fn main() {
    let start = Instant::now();
    let bits = parse("input16.txt");
    println!("Part 1: {}", part1(&bits)); // 2321
    println!("Part 2: {}", part2(&bits)); // 118803
    let elapsed = start.elapsed();
    println!("Elapsed: {}µs", elapsed.as_micros());
}

fn parse(filename: &str) -> String {
    fs::read_to_string(filename).expect("Error reading file!")
}

fn get_packets(hex: &str) -> Packet {
    println!("Hex: {}", hex);
    let bits: String = hex.chars().map(|c| match c {
        '0'..='9' | 'A'..='F' => format!("{:04b}", u8::from_str_radix(&c.to_string(), 16).unwrap()),
        _ => "".to_string(),
    }).collect();

    let (packet, _) = get_packets_bits(&bits);
    packet.unwrap()
}

fn get_packets_bits(bits: &str) -> (Option<Packet>, usize) {
    println!("Bits: {}", bits);

    if bits.len() < 6 {
        println!("Too few bits remaining: {}", bits.len());
        return (None, 0);
    }

    let mut i: usize = 0;
    let starting_bit = i;
    // Version
    let version = u8_from_binary(&bits[i..i+3]);
    println!("Version: {}", version);
    i += 3;

    // TypeID
    let type_id = u8_from_binary(&bits[i..i+3]) as u8;
    println!("Type ID: {}", type_id);
    i += 3;

    let mut literal: Option<usize> = None;
    let mut subpackets = Vec::new();
    match type_id {
        4 => {
            let mut literal_bits = String::new();
            loop {
                let token = &bits[i..i+5];
                i += 5;
                // Concat to literal
                literal_bits.push_str(&token[1..5]);
                if token.starts_with("0") {
                    break;
                }
            }
            literal = Some(from_binary(&literal_bits));
            println!("Literal: {:?}", literal);
        },
        _ => {
            println!("Operator Packet");
            let length_type_id = &bits[i..i+1];
            i += 1;

            if length_type_id == "0" {
                println!("Bit Length modifier");
                let mut subpackets_bit_length = from_binary(&bits[i..i+15]);
                i += 15;
                while subpackets_bit_length > 0 {
                    match get_packets_bits(&bits[i..i+subpackets_bit_length]) {
                        (Some(subpacket), subpacket_bits_used) => {
                            subpackets.push(subpacket);
                            i += subpacket_bits_used;
                            subpackets_bit_length -= subpacket_bits_used;
                        },
                        (None, _) => (),
                    }
                }
            } else {
                println!("Packet count modifier");
                let mut subpacket_count = from_binary(&bits[i..i+11]);
                i += 11;
                while subpacket_count > 0 {
                    match get_packets_bits(&bits[i..]) {
                        (Some(subpacket), subpacket_bits_used) => {
                            subpackets.push(subpacket);
                            i += subpacket_bits_used;
                            subpacket_count -= 1;
                        },
                        (None, _) => (),
                    }
                }
            }
        },
    }
    println!();

    let packet = Some(Packet {version, type_id, literal, subpackets});
    let bits_used = i - starting_bit;
    (packet, bits_used)
}

fn u8_from_binary(s: &str) -> u8 {
    u8::from_str_radix(s, 2).unwrap()
}

fn from_binary(s: &str) -> usize {
    usize::from_str_radix(s, 2).unwrap()
}

fn part1(hex: &str) -> usize {
    let packet = get_packets(hex);
    packet.version as usize
}

fn part2(hex: &str) -> usize {
    0
}


#[test]
fn test_part1a() {
    assert_eq!(part1(&String::from("D2FE28").as_str()), 6);
}

#[test]
fn test_part1b() {
    assert_eq!(part1(&String::from("38006F45291200").as_str()), 1);
}

#[test]
fn test_part1c() {
    assert_eq!(part1(&String::from("8A004A801A8002F478").as_str()), 16);
}

#[test]
fn test_part2a() {
    assert_eq!(part2(&parse("sample.txt")), 315);
}
