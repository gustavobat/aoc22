use anyhow::Result;
use std::str::FromStr;
use aoc22::read_one_per_line;

#[derive(Debug, Copy, Clone)]
enum OpponentPlay {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl FromStr for OpponentPlay {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "A" => Ok(OpponentPlay::Rock),
            "B" => Ok(OpponentPlay::Paper),
            "C" => Ok(OpponentPlay::Scissors),
            _ => Err(anyhow::format_err!("Could not parse OpponentPlay"))
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum SecondColumn {
    X = 0,
    Y = 1,
    Z = 2,
}

impl FromStr for SecondColumn {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "X" => Ok(SecondColumn::X),
            "Y" => Ok(SecondColumn::Y),
            "Z" => Ok(SecondColumn::Z),
            _ => Err(anyhow::format_err!("Could not parse SecondColumn"))
        }
    }
}

impl SecondColumn {
    fn to_shape_points(&self) -> u32 {
        *self as u32 + 1
    }

    fn to_outcome_points(&self) -> u32 {
        *self as u32 * 3
    }
}

#[derive(Debug)]
struct Round {
    opponent_play: OpponentPlay,
    second_column: SecondColumn,
}

impl FromStr for Round {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        if let Some((opponent_play_str, second_column_str)) = s.split_once(" ") {
            Ok(Round::new(
                    OpponentPlay::from_str(opponent_play_str).unwrap(),
                    SecondColumn::from_str(second_column_str).unwrap()
                    )
              )
        } else {
            Err(anyhow::format_err!("Could not parse plays"))
        }
    }
}


impl Round {
    fn new(opponent_play : OpponentPlay, second_column: SecondColumn) -> Self {
        Self{opponent_play, second_column}
    }

    fn outcome_score(&self) -> u32 {
        (self.second_column as u32 + 5 - self.opponent_play as u32) % 3 * 3
    }

    fn shape_score(&self) -> u32 {
        (self.opponent_play as u32 + self.second_column as u32 + 1) % 3 + 1
    }

    pub fn total_round_score_1(&self) -> u32 {
        self.second_column.to_shape_points() + self.outcome_score()
    }

    pub fn total_round_score_2(&self) -> u32 {
        self.second_column.to_outcome_points() + self.shape_score()
    }
}

fn main() -> Result<()> {

    let round_vec = read_one_per_line::<Round>("./data/02.input")?;

    let mut score1 : u32 = 0;
    for round in &round_vec {
        score1 += round.total_round_score_1()
    }

    let mut score2 : u32 = 0;
    for round in &round_vec {
        score2 += round.total_round_score_2()
    }

    println!("Total score 1: {}", score1);
    println!("Total score 2: {}", score2);

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn outcome_score() {
        assert_eq!(Round::new(OpponentPlay::Rock, SecondColumn::X).outcome_score(), 3);
        assert_eq!(Round::new(OpponentPlay::Rock, SecondColumn::Y).outcome_score(), 6);
        assert_eq!(Round::new(OpponentPlay::Rock, SecondColumn::Z).outcome_score(), 0);
        assert_eq!(Round::new(OpponentPlay::Paper, SecondColumn::X).outcome_score(), 0);
        assert_eq!(Round::new(OpponentPlay::Paper, SecondColumn::Y).outcome_score(), 3);
        assert_eq!(Round::new(OpponentPlay::Paper, SecondColumn::Z).outcome_score(), 6);
        assert_eq!(Round::new(OpponentPlay::Scissors, SecondColumn::X).outcome_score(), 6);
        assert_eq!(Round::new(OpponentPlay::Scissors, SecondColumn::Y).outcome_score(), 0);
        assert_eq!(Round::new(OpponentPlay::Scissors, SecondColumn::Z).outcome_score(), 3);
    }
    #[test]
    fn shape_score() {
        assert_eq!(Round::new(OpponentPlay::Rock, SecondColumn::X).shape_score(), 3);
        assert_eq!(Round::new(OpponentPlay::Rock, SecondColumn::Y).shape_score(), 1);
        assert_eq!(Round::new(OpponentPlay::Rock, SecondColumn::Z).shape_score(), 2);
        assert_eq!(Round::new(OpponentPlay::Paper, SecondColumn::X).shape_score(), 1);
        assert_eq!(Round::new(OpponentPlay::Paper, SecondColumn::Y).shape_score(), 2);
        assert_eq!(Round::new(OpponentPlay::Paper, SecondColumn::Z).shape_score(), 3);
        assert_eq!(Round::new(OpponentPlay::Scissors, SecondColumn::X).shape_score(), 2);
        assert_eq!(Round::new(OpponentPlay::Scissors, SecondColumn::Y).shape_score(), 3);
        assert_eq!(Round::new(OpponentPlay::Scissors, SecondColumn::Z).shape_score(), 1);
    }
}
