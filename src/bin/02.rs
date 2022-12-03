use anyhow::Result;
use std::str::FromStr;

#[derive(Debug)]
enum ShapePoints {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(Debug)]
enum OutcomePoints {
    Loss = 0,
    Draw = 3,
    Win = 6,
}

#[derive(Debug)]
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

#[derive(Debug)]
#[repr(u8)]
enum SecondColumn {
    X = b'X',
    Y = b'Y',
    Z = b'Z',
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

#[derive(Debug)]
struct Round {
    opponent_play: OpponentPlay,
    second_column: SecondColumn,
}

impl FromStr for Round {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        if let Some((opponent_play_str, second_column_str)) = s.split_once(" ") {
            Ok(Round{ 
                opponent_play: OpponentPlay::from_str(opponent_play_str).unwrap(),
                second_column: SecondColumn::from_str(second_column_str).unwrap(),
            })
        } else {
            Err(anyhow::format_err!("Could not parse plays"))
        }
    }
}


impl Round {

    fn outcome_score(&self) -> u32 {
        match self.opponent_play {
            OpponentPlay::Rock =>
                match self.second_column {
                    SecondColumn::X => OutcomePoints::Draw as u32,
                    SecondColumn::Y => OutcomePoints::Win as u32,
                    SecondColumn::Z => OutcomePoints::Loss as u32,
                },
            OpponentPlay::Paper =>
                match self.second_column {
                    SecondColumn::X => OutcomePoints::Loss as u32,
                    SecondColumn::Y => OutcomePoints::Draw as u32,
                    SecondColumn::Z => OutcomePoints::Win as u32,
                },
            OpponentPlay::Scissors =>
                match self.second_column {
                    SecondColumn::X => OutcomePoints::Win as u32,
                    SecondColumn::Y => OutcomePoints::Loss as u32,
                    SecondColumn::Z => OutcomePoints::Draw as u32,
                },                     
        }
    }

    fn shape_score(&self) -> u32 {
        match self.opponent_play {
            OpponentPlay::Rock =>
                match self.second_column {
                    SecondColumn::X => ShapePoints::Scissors as u32,
                    SecondColumn::Y => ShapePoints::Rock as u32,
                    SecondColumn::Z => ShapePoints::Paper as u32,
                },
            OpponentPlay::Paper => 
                match self.second_column {
                    SecondColumn::X => ShapePoints::Rock as u32,
                    SecondColumn::Y => ShapePoints::Paper as u32,
                    SecondColumn::Z => ShapePoints::Scissors as u32,
                },
            OpponentPlay::Scissors =>
                match self.second_column {
                    SecondColumn::X => ShapePoints::Paper as u32,
                    SecondColumn::Y => ShapePoints::Scissors as u32,
                    SecondColumn::Z => ShapePoints::Rock as u32,
                },
        }
    }

    pub fn total_round_score_1(&self) -> u32 {
        match self.second_column {
            SecondColumn::X => ShapePoints::Rock as u32 + self.outcome_score(),
            SecondColumn::Y => ShapePoints::Paper as u32 + self.outcome_score(),
            SecondColumn::Z => ShapePoints::Scissors as u32 + self.outcome_score(),
        }
    }

    pub fn total_round_score_2(&self) -> u32 {
        match self.second_column {
            SecondColumn::X => OutcomePoints::Loss as u32 + self.shape_score(),
            SecondColumn::Y => OutcomePoints::Draw as u32 + self.shape_score(),
            SecondColumn::Z => OutcomePoints::Win as u32 + self.shape_score(),
        }
    }
}

fn main() -> Result<()> {
    
    let round_vec : Vec<Round> = std::fs::read_to_string("./data/02.input")?
        .lines()
        .filter_map(|line| line.parse::<Round>().ok())
        .collect();

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
