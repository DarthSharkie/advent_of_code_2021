use std::fs;
use std::time::Instant;


struct Packet {
    version: u8,
    type_id: u8,
    literal: Option<usize>,
    subpackets: Vec<Packet>,
}

impl Packet {
    fn version_sum(&self) -> usize {
        println!("My version: {}", self.version);
        self.subpackets.iter().map(Self::version_sum).sum::<usize>() + self.version as usize
    }

    fn evaluate(&self) -> usize {
        match self.type_id {
            0 => self.subpackets.iter().map(Self::evaluate).sum(),
            1 => self.subpackets.iter().map(Self::evaluate).product(),
            2 => self.subpackets.iter().map(Self::evaluate).min().unwrap(),
            3 => self.subpackets.iter().map(Self::evaluate).max().unwrap(),
            4 => self.literal.unwrap(),
            5 => if self.subpackets[0].evaluate() > self.subpackets[1].evaluate() { 1 } else { 0 },
            6 => if self.subpackets[0].evaluate() < self.subpackets[1].evaluate() { 1 } else { 0 },
            7 => if self.subpackets[0].evaluate() == self.subpackets[1].evaluate() { 1 } else { 0 },
            _ => 0,
        }
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
    packet.version_sum()
}

fn part2(hex: &str) -> usize {
    let packet = get_packets(hex);
    packet.evaluate()
}


#[test]
fn test_part1a() {
    assert_eq!(part1(&String::from("D2FE28").as_str()), 6);
    assert_eq!(part1(&String::from("38006F45291200").as_str()), 9);
    assert_eq!(part1(&String::from("8A004A801A8002F478").as_str()), 16);
    assert_eq!(part1(&String::from("620080001611562C8802118E34").as_str()), 12);
    assert_eq!(part1(&String::from("C0015000016115A2E0802F182340").as_str()), 23);
    assert_eq!(part1(&String::from("A0016C880162017C3686B18A3D4780").as_str()), 31);
}

#[test]
fn test_part2a() {
    assert_eq!(part2(&String::from("C200B40A82")), 3); // 1 + 2
    assert_eq!(part2(&String::from("04005AC33890")), 54);  // 6 * 9
    assert_eq!(part2(&String::from("880086C3E88112")), 7);  // min(7, 8, 9)
    assert_eq!(part2(&String::from("CE00C43D881120")), 9);  // max(7, 8, 9)
    assert_eq!(part2(&String::from("D8005AC2A8F0")), 1);    // 5 < 15
    assert_eq!(part2(&String::from("F600BC2D8F")), 0);      // 5 > 15
    assert_eq!(part2(&String::from("9C005AC2F8F0")), 0);    // 5 != 15
    assert_eq!(part2(&String::from("9C0141080250320F1802104A08")), 1); // 1 + 3 = 2 * 2
}
