use std::fs;
use std::str::FromStr;
use std::collections::HashMap;

fn main() {
    println!("Part 1: {}", part1(&parse("input08.txt"))); // 543
    println!("Part 2: {}", part2(&parse("input08.txt"))); // 994266
}

fn parse(filename: &str) -> Vec<Entry> {
    let contents = fs::read_to_string(filename).expect("Error reading file!");
    contents.trim().lines().map(str::trim).map(str::parse).map(Result::unwrap).collect()
}

struct Entry {
	patterns: Vec<String>,
	encoded_output: Vec<String>,
}

impl FromStr for Entry {
	type Err = String;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let (input, output) = s.split_once('|').expect("Malformed input");
        let patterns = input.trim()
            .split(' ')
            .map(|s| {
                let mut t = s.chars().collect::<Vec<char>>();
                t.sort();
                t})
            .map(String::from_iter)
            .collect();
        let encoded_output = output.trim()
            .split(' ')
            .map(|s| {
                let mut t = s.chars().collect::<Vec<char>>();
                t.sort();
                t})
            .map(String::from_iter)
            .collect();
        Ok(Entry { patterns, encoded_output })
	}
}

impl Entry {
    fn decode(&self) -> i32 {
        let code = self.deduce();
        self.encoded_output.iter().map(|eo| code.get(eo).expect("no mapping")).fold(0, |out, digit| out*10 + digit)
    }

    fn deduce(&self) -> HashMap<String, i32> {
        let encodings = [self.patterns.clone(), self.encoded_output.clone()].concat(); 
        let mut code = HashMap::new();

        // Simple identifications, only one digit with this number of segments lit
        let eight: String = encodings.iter().find(|&s| s.len() == 7).unwrap().to_string();
        let one: String = encodings.iter().find(|&s| s.len() == 2).unwrap().to_string();
        let seven: String = encodings.iter().find(|&s| s.len() == 3).unwrap().to_string();
        let four: String = encodings.iter().find(|&s| s.len() == 4).unwrap().to_string();

        // Nine is the only six-segment containing all of Four
        let nine: String = encodings.iter().find(|&s| s.len() == 6 && self.contains_all(s, &four)).unwrap().to_string();
        // G = Nine - Four - Seven
        let g: char = nine.chars().find(|&h| !four.contains(h) && !seven.contains(h)).unwrap();
        // E = Eight - Nine
        let e: char = eight.chars().find(|&h| !nine.contains(h)).unwrap();

        // Three is the only five-segment containing all of Seven
        let three: String = encodings.iter().find(|&s| s.len() == 5 && self.contains_all(&s, &seven)).unwrap().to_string();
        // D = Three - Seven - G
        let d: char = three.chars().find(|&h| !seven.contains(h) && h != g).unwrap();
        // B = Nine - Three
        let b: char = nine.chars().find(|&h| !three.contains(h)).unwrap();

        // Six is the only six-segment containing the proper segments of D and E
        let six: String = encodings.iter().find(|&s| s.len() == 6 && s.contains(d) && s.contains(e)).unwrap().to_string();

        // Zero is the only six-segment NOT containing the proper segment D
        let zero: String = encodings.iter().find(|&s| s.len() == 6 && !s.contains(d)).unwrap().to_string();
        // Two is the only five-segment containing the proper segment E
        let two: String = encodings.iter().find(|&s| s.len() == 5 && s.contains(e)).unwrap().to_string();
        // Five is the only five-segment containing the proper segment B
        let five: String = encodings.iter().find(|&s| s.len() == 5 && s.contains(b)).unwrap().to_string();

        code.insert(zero, 0);
        code.insert(one, 1);
        code.insert(two, 2);
        code.insert(three, 3);
        code.insert(four, 4);
        code.insert(five, 5);
        code.insert(six, 6);
        code.insert(seven, 7);
        code.insert(eight, 8);
        code.insert(nine, 9);
        code
    }

    fn contains_all(&self, haystack: &str, needle: &str) -> bool {
        needle.chars().find(|&c| !haystack.contains(c)) == None
    }
}

fn part1(entries: &[Entry]) -> i32 {
    entries.iter().flat_map(|entry| entry.encoded_output.iter()).filter(|&o| o.len() != 5 && o.len() != 6).count() as i32
}

fn part2(entries: &[Entry]) -> i32 {
    entries.iter().map(Entry::decode).sum()
}


#[test]
fn test_part1() {
    assert_eq!(part1(&parse("sample.txt")), 26);
}

#[test]
fn test_part2() {
    assert_eq!(part2(&parse("sample.txt")), 61229);
}
