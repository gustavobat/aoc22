use anyhow::Result;
use std::str::FromStr;

#[derive(Clone, Copy, Debug)]
#[repr(u8)]
enum OpponentPlay {
    Rock = b'A',
    Paper = b'B',
    Scissors = b'C',
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

#[derive(Clone, Copy, Debug)]
#[repr(u8)]
enum MyPlay {
    Rock = b'X',
    Paper = b'Y',
    Scissors = b'Z',
}

impl FromStr for MyPlay {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "X" => Ok(MyPlay::Rock),
            "Y" => Ok(MyPlay::Paper),
            "Z" => Ok(MyPlay::Scissors),
            _ => Err(anyhow::format_err!("Could not parse MyPlay"))
        }
    }
}

#[derive(Debug)]
struct Round {
    opponent_play: OpponentPlay,
    my_play: MyPlay,
}

impl FromStr for Round {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        if let Some((opponent_play_str, my_play_str)) = s.split_once(" ") {
            Ok(Round{ 
                opponent_play: OpponentPlay::from_str(opponent_play_str).unwrap(),
                my_play: MyPlay::from_str(my_play_str).unwrap(),
            })
        } else {
            Err(anyhow::format_err!("Could not parse plays"))
        }
    }
}

impl Round {

    fn round_outcome_score(&self) -> u32 {
        match self.opponent_play {
            OpponentPlay::Rock =>
                match self.my_play {
                    MyPlay::Rock => 3,
                    MyPlay::Paper => 6,
                    MyPlay::Scissors => 0,
                },
            OpponentPlay::Paper =>
                match self.my_play {
                    MyPlay::Rock => 0,
                    MyPlay::Paper => 3,
                    MyPlay::Scissors => 6,
                },
            OpponentPlay::Scissors =>
                match self.my_play {
                    MyPlay::Rock => 6,
                    MyPlay::Paper => 0,
                    MyPlay::Scissors => 3,
                },
        }
    }

    pub fn total_round_score(&self) -> u32 {
        match self.my_play {
            MyPlay::Rock => 1 + self.round_outcome_score(),
            MyPlay::Paper => 2 + self.round_outcome_score(),
            MyPlay::Scissors => 3 + self.round_outcome_score(),
        }
    }
}

fn main() -> Result<()> {
    let round_vec : Vec<Round> = std::fs::read_to_string("./data/02.input")?
        .lines()
        .filter_map(|line| line.parse::<Round>().ok())
        .collect();

    let mut score : u32 = 0;
    for round in round_vec {
        score += round.total_round_score()
    }
    
    println!("Total score: {}", score);

    Ok(())
}
