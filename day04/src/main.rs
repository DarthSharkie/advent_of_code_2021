use std::fs;

// Replace with day-specific struct/enums
struct Card {
    slots: [[u32; 5]; 5],
    marks: [[bool; 5]; 5],
    score: u32,
	draws: u32,
}

impl Card {
    fn from_chunk(chunk: &str) -> Card {
        let mut board: [[u32; 5]; 5] = [[0; 5]; 5];
        for (i, line) in chunk.lines().enumerate() {
            let row: Vec<u32> = line.split_whitespace().map(|num| num.parse().unwrap()).collect();
            for (j, num) in row.iter().enumerate() {
                board[i][j] = *num;
            }
        }
        Card { slots: board, marks: [[false; 5]; 5], score: 0, draws: 0 }
    }

	fn play(&mut self, moves: &[u32]) {
		for mv in moves {
			self.mark(mv);
			if self.score > 0 {
                break;
			}
		}
	}


    fn mark(&mut self, value: &u32) {
		if self.score > 0 {
			return
		}
		self.draws += 1;
        for i in 0..5 {
            for j in 0..5 {
                if self.slots[i][j] == *value {
                    self.marks[i][j] = true;
                    self.score = value * self.evaluate();
                    return
                }
            }
        }
    }

    fn score(&self) -> u32 {
        self.score
    }

    fn evaluate(&self) -> u32 {
        let mut row_wins: [bool; 5] = [true; 5];
        let mut col_wins: [bool; 5] = [true; 5];
        for (i, row_win) in row_wins.iter_mut().enumerate() {
            for (j, col_win) in col_wins.iter_mut().enumerate() {
                *row_win &= self.marks[i][j];
                *col_win &= self.marks[i][j];
            }
        }

        if row_wins.iter().any(|&row| row) || col_wins.iter().any(|&col| col) {
			let values = self.slots.iter().flat_map(|row| row.iter());
			let marked = self.marks.iter().flat_map(|row| row.iter());
			values.zip(marked).map(|(&value, &mark)| if mark {0u32} else {value}).sum()
        } else {
            0
        }
    }

}

fn main() {
    let filename = String::from("input04.txt");
    let contents = fs::read_to_string(filename).expect("Error reading file!");
    
    let (moves, boards) = contents.split_once("\n\n").unwrap();
	let moves: Vec<u32> = moves.split(',').map(|num| num.parse().unwrap()).collect();

    println!("Part 1: {}", part1(&boards, &moves));
    println!("Part 2: {}", part2(&boards, &moves));

}

fn part1(boards: &str, moves: &[u32]) -> u32 {
    let mut cards: Vec<Card> = boards.split("\n\n").map(Card::from_chunk).collect();

    cards.iter_mut().for_each(|card| card.play(moves));
    cards.sort_by(|lhs, rhs| lhs.draws.cmp(&rhs.draws));
    cards[0].score()
}

fn part2(boards: &str, moves: &[u32]) -> u32 {
    let mut cards: Vec<Card> = boards.split("\n\n").map(Card::from_chunk).collect();

    cards.iter_mut().for_each(|card| card.play(moves));
    cards.sort_by(|lhs, rhs| lhs.draws.cmp(&rhs.draws));
    cards[cards.len() - 1].score()
}
