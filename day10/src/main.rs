use std::fs;
use std::str::FromStr;
use std::collections::VecDeque;
use std::collections::HashMap;

type Nav = Vec<Vec<char>>;

fn main() {
    println!("Part 1: {}", part1(&parse("input10.txt"))); // 518
    println!("Part 2: {}", part2(&parse("input10.txt"))); // 949905
}

fn parse(filename: &str) -> Nav {
    let contents = fs::read_to_string(filename).expect("Error reading file!");
    contents.trim().lines().map(|s| {
		s.chars().collect()
    }).collect()
}

fn part1(nav: &Nav) -> i32 {
    let mut counts: HashMap<char, i32> = HashMap::new();
	for cmd in nav {
        let mut stack = VecDeque::new();
        for ins in cmd {
            let expected = match ins {
                '[' | '(' | '{' | '<' => { stack.push_front(ins); None },
                '>' => if let Some('<') = stack.get(0) {
                    stack.pop_front();
                    None
                } else {
                    Some(ins)
                },
                '}' => if let Some('{') = stack.get(0) {
                    stack.pop_front();
                    None
                } else {
                    Some(ins)
                },
                ')' => if let Some('(') = stack.get(0) {
                    stack.pop_front();
                    None
                } else {
                    Some(ins)
                },
                ']' => if let Some('[') = stack.get(0) {
                    stack.pop_front();
                    None
                } else {
                    Some(ins)
                },
                _ => panic!("Shouldn't happen"),
            };
            match expected {
                Some(close) => {
                    println!("Mismatch: {}", close);
                    let entry = counts.entry(*close).or_insert(0);
                    *entry += 1;
                    break;
                },
                _ => (),
            }
        }
        if stack.len() > 0 {
            println!("Incomplete!");
        }
	}
    println!("Counts: {:?}", counts);
    (*counts.entry(')').or_default() * 3) + 
    (*counts.entry(']').or_default() * 57) + 
    (*counts.entry('}').or_default() * 1197) + 
    (*counts.entry('>').or_default() * 25137) 
}

fn part2(nav: &Nav) -> usize {
    let mut scores: Vec<usize> = Vec::new();
	for cmd in nav {
        let mut stack = VecDeque::new();
        let mut corrupted: bool = false;
        for ins in cmd {
            let expected = match ins {
                '[' | '(' | '{' | '<' => { stack.push_front(ins); None },
                '>' => if let Some('<') = stack.get(0) {
                    stack.pop_front();
                    None
                } else {
                    Some(ins)
                },
                '}' => if let Some('{') = stack.get(0) {
                    stack.pop_front();
                    None
                } else {
                    Some(ins)
                },
                ')' => if let Some('(') = stack.get(0) {
                    stack.pop_front();
                    None
                } else {
                    Some(ins)
                },
                ']' => if let Some('[') = stack.get(0) {
                    stack.pop_front();
                    None
                } else {
                    Some(ins)
                },
                _ => panic!("Shouldn't happen"),
            };
            match expected {
                Some(_) => corrupted = true,
                _ => (),
            }
        }
        if stack.len() > 0 && !corrupted {
            let mut score: usize = 0;
            println!("Stack: {:?}", stack);
            while !stack.is_empty() {
                let points = match stack.pop_front() {
                    Some('(') => 1,
                    Some('[') => 2,
                    Some('{') => 3,
                    Some('<') => 4,
                    _ => panic!("Bad!"),
                };
                score = score * 5 + points;
            }
            scores.push(score);
        }
	}
    scores.sort_unstable();
    println!("Scores: {:?}", scores);
    scores[(scores.len()-1)/2]
}



#[test]
fn test_part1() {
    assert_eq!(part1(&parse("sample.txt")), 26397);
}

#[test]
fn test_part2() {
    assert_eq!(part2(&parse("sample.txt")), 288957);
}
