use std::path::Path;
use crate::{err::Error, read_two_string_cols};

#[derive(Debug)]
#[repr(u8)]
pub enum RockPaperScissors {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl RockPaperScissors {
    pub fn from_char(input: char) -> Option<Self> {
        match input {
            'A' => Some(Self::Rock),
            'X' => Some(Self::Rock),
            'B' => Some(Self::Paper),
            'Y' => Some(Self::Paper),
            'C' => Some(Self::Scissors),
            'Z' => Some(Self::Scissors),
            _ => None,
        }
    }

    pub fn value(&self) -> i32 {
        match *self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    pub fn scores(&self, other: &RockPaperScissors) -> (i32, i32) {
        let diff = self.value() - other.value();
        if diff == 0 {
            // Draw
            println!("{}, Draw", diff);
            (
                self.value() + RPSResult::Draw.value(),
                other.value() + RPSResult::Draw.value(),
            )
        } else {
            let check_val =
                (diff.signum() as f64 * ((-1.0_f64).powi(diff).round() * -1_f64)) as i32;

            if check_val == 1 {
                // Self (Player 1) wins.
                println!("{}, P1 win", diff);
                (
                    self.value() + RPSResult::Win.value(),
                    other.value() + RPSResult::Loss.value(),
                )
            } else {
                // Other (player 2) wins.
                println!("{}, P2 win", diff);
                (
                    self.value() + RPSResult::Loss.value(),
                    other.value() + RPSResult::Win.value(),
                )
            }
        }
    }
}

impl From<usize> for RockPaperScissors {
    fn from(num: usize) -> Self {
        match num {
            1 => Self::Rock,
            2 => Self::Paper,
            3 => Self::Scissors,
            _ => panic!(),
        }
    }
}

pub enum RPSResult {
    Win = 6,
    Draw = 3,
    Loss = 0,
}

impl RPSResult {
    pub fn value(&self) -> i32 {
        match *self {
            Self::Win => 6,
            Self::Draw => 3,
            Self::Loss => 0,
        }
    }

    pub fn from_char(input: char) -> Option<Self> {
        match input {
            'X' => Some(Self::Loss),
            'Y' => Some(Self::Draw),
            'Z' => Some(Self::Win),
            _ => None,
        }
    }

    pub fn piece_offset(&self) -> i32 {
        match *self {
            Self::Win => 1,
            Self::Draw => 0,
            Self::Loss => -1,
        }
    }
}

pub enum RPSPlayer {
    Player1,
    Player2,
}

#[derive(Debug)]
pub struct RPSGame {
    pub rounds: Vec<(RockPaperScissors, RockPaperScissors)>,
}

impl RPSGame {
    pub fn tot_scores(&self) -> (i32, i32) {
        let results = self
            .rounds
            .iter()
            .map(|(p1, p2)| p1.scores(p2))
            .collect::<Vec<(i32, i32)>>();
        let p1_score = results.iter().map(|r| r.0).sum();
        let p2_score = results.iter().map(|r| r.1).sum();
        (p1_score, p2_score)
    }
}

#[inline]
pub fn load_game(path: &Path) -> Result<RPSGame, Error> {
    let (col1, col2) = read_two_string_cols::<' '>(path)?;

    let rounds = col1
        .iter()
        .zip(&col2)
        .map(|(p1_str, p2_str)| {
            let p1 =
                RockPaperScissors::from_char(p1_str.chars().collect::<Vec<char>>()[0]).unwrap();
            let p2 =
                RockPaperScissors::from_char(p2_str.chars().collect::<Vec<char>>()[0]).unwrap();

            (p1, p2)
        })
        .collect();

    Ok(RPSGame { rounds })
}

#[inline]
pub fn load_p1_and_results(path: &Path) -> Result<RPSGame, Error> {
    let (col1, col2) = read_two_string_cols::<' '>(path)?;

    let rounds = col1
        .iter()
        .zip(&col2)
        .map(|(p1_str, res_str)| {
            let p1 =
                RockPaperScissors::from_char(p1_str.chars().collect::<Vec<char>>()[0]).unwrap();
            let res = RPSResult::from_char(res_str.chars().collect::<Vec<char>>()[0]).unwrap();

            let val = res.piece_offset() + p1.value();
            let p2: RockPaperScissors = (if val == 0 {
                3
            } else if val == 4 {
                1
            } else {
                val
            } as usize)
                .into();

            (p1, p2)
        })
        .collect();

    Ok(RPSGame { rounds })
}
