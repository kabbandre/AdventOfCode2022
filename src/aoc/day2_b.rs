use std::fs::read_to_string;

enum GameOutcome {
    Lose,
    Draw,
    Win
}

impl GameOutcome {
    fn create_from_str(move_option: &str) -> Option<GameOutcome> {
        match move_option {
            "X" => Some(GameOutcome::Lose),
            "Y" => Some(GameOutcome::Draw),
            "Z" => Some(GameOutcome::Win),
            _ => None,
        }
    }
    fn get_score(&self) -> i32 {
        match self {
            GameOutcome::Lose => 0,
            GameOutcome::Draw => 3,
            GameOutcome::Win => 6,
        }
    }
}

#[derive(Clone)]
enum Move {
    Rock,
    Scissors,
    Paper
}

impl Move {
    fn create_from_str(move_option: &str) -> Option<Move> {
        match move_option {
            "A" => Some(Move::Rock),
            "B" => Some(Move::Paper),
            "C" => Some(Move::Scissors),
            _ => None,
        }
    }
    fn get_score(&self) -> i32 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }
}

struct Round {
    opponent: Move,
    expected_outcome: GameOutcome
}

impl Round {
    fn play(&self) -> i32 {
        self.get_player_move().get_score() + self.expected_outcome.get_score()
    }
    fn get_player_move(&self) -> Move {
        match self.expected_outcome {
            GameOutcome::Draw => self.opponent.clone(),
            GameOutcome::Lose => match self.opponent {
                Move::Rock => Move::Scissors,
                Move::Scissors => Move::Paper,
                Move::Paper => Move::Rock,
            },
            GameOutcome::Win => match self.opponent {
                Move::Rock => Move::Paper,
                Move::Scissors => Move::Rock,
                Move::Paper => Move::Scissors,
            },
        }
    }
}

pub fn solution() {
    let mut result = 0;
    let contents = read_to_string("./assets/day2.txt").unwrap();

    for line in contents.lines() {
        let mut moves = line.split_whitespace();
        let opponent = Move::create_from_str(moves.next().unwrap()).unwrap();
        let expected_outcome = GameOutcome::create_from_str(moves.next().unwrap()).unwrap();
        let round = Round {
            opponent,
            expected_outcome
        };
        result += round.play();
    }

    println!("Day 2 Part 2 result - {}", result)
}