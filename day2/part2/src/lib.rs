use std::str::Lines;

#[derive(Debug, PartialEq, Eq)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    pub fn from_char(move_char: &str) -> Self {
        match move_char {
            "A" => Move::Rock,
            "B" => Move::Paper,
            "C" => Move::Scissors,
            c => {
                panic!("Unexpected move code: \'{c}\'")
            }
        }
    }

    pub fn from(round_str: &str) -> (Self, Self) {
        let move_strs: Vec<&str> = round_str.split(' ').collect();

        let this_move = Self::from_char(move_strs[0]);
        let op_move = Self::from_char(move_strs[1]);

        (this_move, op_move)
    }

    /// Get the appropriate move for the intended outcome
    pub fn from_outcome(op_move: &Self, outcome: &Outcome) -> Self {
        match outcome {
            Outcome::Win => {
                // Provide winning moves
                match op_move {
                    Move::Rock => Move::Paper,
                    Move::Paper => Move::Scissors,
                    Move::Scissors => Move::Rock,
                }
            }
            Outcome::Lose => {
                // Provide losing moves
                match op_move {
                    Move::Rock => Move::Scissors,
                    Move::Paper => Move::Rock,
                    Move::Scissors => Move::Paper,
                }
            }
            Outcome::Draw => {
                // Provide tieing moves
                match op_move {
                    Move::Rock => Move::Rock,
                    Move::Paper => Move::Paper,
                    Move::Scissors => Move::Scissors,
                }
            }
        }
    }

    pub fn score(&self) -> u32 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Outcome {
    Win,
    Lose,
    Draw,
}

impl Outcome {
    pub fn from(this_move: &Move, op_move: &Move) -> Self {
        match this_move {
            Move::Rock => {
                return match op_move {
                    Move::Rock => Outcome::Draw,
                    Move::Paper => Outcome::Lose,
                    Move::Scissors => Outcome::Win,
                }
            }
            Move::Paper => {
                return match op_move {
                    Move::Rock => Outcome::Win,
                    Move::Paper => Outcome::Draw,
                    Move::Scissors => Outcome::Lose,
                }
            }
            Move::Scissors => {
                return match op_move {
                    Move::Rock => Outcome::Lose,
                    Move::Paper => Outcome::Win,
                    Move::Scissors => Outcome::Draw,
                }
            }
        }
    }

    pub fn from_code(code: &str) -> Self {
        match code {
            "X" => Outcome::Lose,
            "Y" => Outcome::Draw,
            "Z" => Outcome::Win,
            c => {
                panic!("Unexpected outcome code: \'{c}\'")
            }
        }
    }

    pub fn score(&self) -> u32 {
        match self {
            Outcome::Win => 6,
            Outcome::Lose => 0,
            Outcome::Draw => 3,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct RpsRound {
    opponent_move: Move,
    recommended_move: Move,
    outcome: Outcome,
}

impl RpsRound {
    pub fn from(round_str: &str) -> Self {
        let moves = Move::from(round_str);
        let outcome = Outcome::from(&moves.1, &moves.0);

        RpsRound {
            opponent_move: moves.0,
            recommended_move: moves.1,
            outcome: outcome,
        }
    }

    pub fn from_lines(round_lines: Lines) -> Vec<Self> {
        let mut rps_rounds: Vec<Self> = Vec::new();

        for line in round_lines {
            rps_rounds.push(RpsRound::from(line));
        }

        rps_rounds
    }

    pub fn from_2(round_str: &str) -> Self {
        let round_fields: Vec<&str> = round_str.split(" ").collect();
        let op_move = Move::from_char(round_fields[0]);
        let outcome = Outcome::from_code(round_fields[1]);

        let this_move = Move::from_outcome(&op_move, &outcome);

        RpsRound {
            opponent_move: op_move,
            recommended_move: this_move,
            outcome,
        }
    }

    pub fn from_lines_2(round_lines: Lines) -> Vec<Self> {
        let mut rps_rounds: Vec<Self> = Vec::new();

        for line in round_lines {
            rps_rounds.push(RpsRound::from_2(line));
        }

        rps_rounds
    }

    pub fn score(&self) -> u32 {
        self.recommended_move.score() + self.outcome.score()
    }

    pub fn total_score(rounds: Vec<Self>) -> u32 {
        let mut total = 0;

        for round in rounds {
            total += round.score();
        }

        total
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_move_from_char() {
        let test_data = vec![
            ("A", Move::Rock),
            ("X", Move::Rock),
            ("B", Move::Paper),
            ("Y", Move::Paper),
            ("C", Move::Scissors),
            ("Z", Move::Scissors),
        ];

        for data in test_data {
            assert_eq!(data.1, Move::from_char(data.0));
        }
    }

    #[test]
    fn create_moves_from_string() {
        let test_data = vec![
            ("A Y", Move::Rock, Move::Paper),
            ("B X", Move::Paper, Move::Rock),
            ("C Z", Move::Scissors, Move::Scissors),
        ];

        for data in test_data {
            assert_eq!((data.1, data.2), Move::from(data.0));
        }
    }

    #[test]
    fn create_outcome_from_moves() {
        let test_data = vec![
            (Move::Rock, Move::Rock, Outcome::Draw),
            (Move::Paper, Move::Rock, Outcome::Win),
            (Move::Scissors, Move::Rock, Outcome::Lose),
            (Move::Scissors, Move::Scissors, Outcome::Draw),
            (Move::Rock, Move::Scissors, Outcome::Win),
            (Move::Paper, Move::Scissors, Outcome::Lose),
        ];

        for data in test_data {
            assert_eq!(data.2, Outcome::from(&data.0, &data.1));
        }
    }

    #[test]
    fn create_outcome_from_code() {
        let test_data = vec![
            ("X", Outcome::Lose),
            ("Y", Outcome::Draw),
            ("Z", Outcome::Win),
        ];

        for data in test_data {
            assert_eq!(data.1, Outcome::from_code(data.0));
        }
    }

    #[test]
    fn create_round_from_string() {
        let input = "A Y";

        let expected = RpsRound {
            opponent_move: Move::Rock,
            recommended_move: Move::Paper,
            outcome: Outcome::Win,
        };

        assert_eq!(expected, RpsRound::from(input));
    }

    #[test]
    fn score_for_round() {
        let input = "A Y";

        let round = RpsRound::from(input);

        assert_eq!(8, round.score());
    }

    #[test]
    fn total_round_scores() {
        let input = "\
A Y
B X
C Z";

        let rounds = RpsRound::from_lines(input.lines());

        assert_eq!(15, RpsRound::total_score(rounds));
    }
}
