use std::fs;

// Replace with day-specific struct/enums
struct Card {
    slots: [[u32; 5]; 5],
    score: u32,
}

impl Card {
    fn from_chunk(chunk: Vec<&str>) -> Card {
        let mut board: [[u32; 5]; 5] = [[0; 5]; 5];
        for (i, line) in chunk.iter().enumerate() {
            let row: Vec<u32> = line.split_whitespace().map(|num| num.parse().unwrap()).collect();
            for (j, num) in row.iter().enumerate() {
                board[i][j] = *num;
            }
        }
        Card { slots: board, score: 0 }
    }

    fn mark(&mut self, value: &u32) {
        for i in 0..5 {
            for j in 0..5 {
                if self.slots[i][j] == *value {
                    self.slots[i][j] = 0;
                }
            }
        }
        self.score = self.evaluate();
    }

    fn score(&self) -> u32 {
        self.score
    }

    fn evaluate(&self) -> u32 {
        let mut row_sums: [u32; 5] = [0; 5];
        let mut col_sums: [u32; 5] = [0; 5];
        for i in 0..5 {
            for j in 0..5 {
                row_sums[i] += self.slots[i][j];
                col_sums[j] += self.slots[i][j];
            }
        }

        if row_sums.iter().min().unwrap() == &0u32 || col_sums.iter().min().unwrap() == &0u32 {
            row_sums.iter().sum::<u32>()
        } else {
            0
        }
    }

}

fn main() {
    let filename = String::from("input04.txt");
    let contents = fs::read_to_string(filename).expect("Error reading file!");
    
    let lines: Vec<&str> = contents.trim().lines().map(str::trim).collect();
    let moves: Vec<u32> = lines[0].split(',').map(|num| num.parse().unwrap()).collect();

    part1(&lines, &moves);
    part2(&lines, &moves);
}

fn part1(lines: &Vec<&str>, moves: &[u32]) {
    let mut cards: Vec<Card> = Vec::new();
    for chunk in lines[2..].chunks(6) {
        cards.push(Card::from_chunk(chunk[0..5].to_vec()));
    }

    'outer: for mv in moves {
        for card in &mut cards {
            card.mark(&mv);
            let sum = card.score();
            if sum > 0 {
                println!("Value: {}, Sum: {}", mv, sum);
                break 'outer;
            }
        }
    }
}

fn part2(lines: &Vec<&str>, moves: &[u32]) {
    let mut cards: Vec<Card> = Vec::new();
    for chunk in lines[2..].chunks(6) {
        cards.push(Card::from_chunk(chunk[0..5].to_vec()));
    }

    for mv in moves {
        for (i, card) in cards.iter_mut().enumerate() {
            card.mark(&mv);
            let score = card.score();
            if score > 0 {
                println!("Card eliminated: {:?}, Score: {}", card.slots, mv * score);
            }
        }
        cards.retain(|card| card.score() == 0);
    }
}

