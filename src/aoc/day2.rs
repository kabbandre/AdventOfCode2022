use std::fs::read_to_string;

enum GameOutcome {
    Lose,
    Draw,
    Win
}

impl GameOutcome {
    fn get_score(&self) -> i32 {
        match self {
            GameOutcome::Lose => 0,
            GameOutcome::Draw => 3,
            GameOutcome::Win => 6,
        }
    }
}

enum Move {
    Rock,
    Scissors,
    Paper
}

impl Move {
    fn create_from_str(move_option: &str) -> Option<Move> {
        match move_option {
            "A" | "X" => Some(Move::Rock),
            "B" | "Y" => Some(Move::Paper),
            "C" | "Z" => Some(Move::Scissors),
            _ => None,
        }
    }
    fn get_shape_score(&self) -> i32 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }
    fn get_outcome_against(&self, opponent: &Move) -> GameOutcome {
        match self {
            Move::Rock => match opponent {
                Move::Rock => GameOutcome::Draw,
                Move::Scissors => GameOutcome::Win,
                Move::Paper => GameOutcome::Lose,
            },
            Move::Scissors => match opponent {
                Move::Scissors => GameOutcome::Draw,
                Move::Paper => GameOutcome::Win,
                Move::Rock => GameOutcome::Lose,
            },
            Move::Paper => match opponent {
                Move::Paper => GameOutcome::Draw,
                Move::Rock => GameOutcome::Win,
                Move::Scissors => GameOutcome::Lose,
            },
        }
    }
}

struct Round {
    opponent: Move,
    player: Move
}

impl Round {
    fn play(&self) -> i32 {
        self.player.get_outcome_against(&self.opponent).get_score() + self.player.get_shape_score()
    }
}

pub fn solution() {
    let mut result = 0;
    let contents = read_to_string("./assets/day2.txt").unwrap();

    for line in contents.lines() {
        let mut moves = line.split_whitespace();
        let opponent = Move::create_from_str(moves.next().unwrap()).unwrap();
        let player = Move::create_from_str(moves.next().unwrap()).unwrap();
        let round = Round {
            opponent,
            player
        };
        result += round.play();
    }

    println!("Day 2 result - {}", result)
}