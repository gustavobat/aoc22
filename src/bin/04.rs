use anyhow::Result;
use std::str::FromStr;
use aoc22::read_one_per_line;

#[derive(Debug)]
struct Assignment {
    start : u8,
    end : u8,
}

impl FromStr for Assignment {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        if let Some((start_str, end_str)) = s.split_once("-") {
             let start : u8 = start_str
                 .parse::<u8>()
                 .expect("Error parsing starting section");

             let end : u8 = end_str
                 .parse::<u8>()
                 .expect("Error parsing ending section");

             Ok(Assignment{ start, end})

        } else {
            Err(anyhow::format_err!("Error spliting sections"))
        }
    }
}

impl Assignment {
    fn fully_contains(&self, assignment: &Assignment) -> bool {
        assignment.start >= self.start && assignment.end <= self.end
    }

    fn overlaps(&self, assignment: &Assignment) -> bool {
        &assignment.start >= &self.start && &assignment.start <= &self.end ||
        &assignment.end >= &self.start && &assignment.end <= &self.end
    }
}

#[derive(Debug)]
struct AssignmentPair {
    first : Assignment,
    second : Assignment,
}

impl FromStr for AssignmentPair {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        if let Some((first_str, second_str)) = s.split_once(",") {
             let first = first_str
                 .parse::<Assignment>()
                 .expect("Error parsing first assignment ");

             let second = second_str
                 .parse::<Assignment>()
                 .expect("Error parsing second assignment ");

             Ok(AssignmentPair{ first, second})

        } else {
            Err(anyhow::format_err!("Error spliting sections"))
        }
    }
}

impl AssignmentPair {
    pub fn fully_contains(&self) -> bool {
        self.first.fully_contains(&self.second) || self.second.fully_contains(&self.first)
    }

    pub fn overlaps(&self) -> bool {
        self.first.overlaps(&self.second) || self.second.overlaps(&self.first)
    }
}

fn count_fully_contains(path : &str) -> Result<usize> {
    Ok(read_one_per_line::<AssignmentPair>(path)?
       .into_iter()
       .filter(|pair| pair.fully_contains())
       .count())
}

fn count_overlaps(path : &str) -> Result<usize> {
    Ok(read_one_per_line::<AssignmentPair>(path)?
       .into_iter()
       .filter(|pair| pair.overlaps())
       .count())
}

fn main() -> Result<()> {
    
    let part1 : usize = count_fully_contains("./data/04.input")
        .expect("Error processing pt. 1");

    let part2 : usize = count_overlaps("./data/04.input")
        .expect("Error processing pt. 2");

    println!("Fully contains count: {}", part1);
    println!("Overlaps count: {}", part2);

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn assignment_from_str() {
        let assignment = Assignment::from_str("2-3").expect("Error parsing assignment");
        assert_eq!(assignment.start, 2);
        assert_eq!(assignment.end, 3);
    }

    #[test]
    fn assignment_pair_from_str() {
        let assignment_pair = AssignmentPair::from_str("2-3,4-6")
            .expect("Error parsing assignment pair");
        assert_eq!(assignment_pair.first.start, 2);
        assert_eq!(assignment_pair.second.end, 6);
    }

    #[test]
    fn fully_contains() {
        assert_eq!(AssignmentPair::from_str("1-1,1-1").unwrap().fully_contains(), true);
        assert_eq!(AssignmentPair::from_str("1-2,1-2").unwrap().fully_contains(), true);
        assert_eq!(AssignmentPair::from_str("1-9,4-5").unwrap().fully_contains(), true);
        assert_eq!(AssignmentPair::from_str("4-5,1-9").unwrap().fully_contains(), true);
        assert_eq!(AssignmentPair::from_str("1-5,3-7").unwrap().fully_contains(), false);
    }

    #[test]
    fn part1() {
        assert_eq!(count_fully_contains("./data/04.test").unwrap(), 2);
    }

    #[test]
    fn overlaps() {
        assert_eq!(AssignmentPair::from_str("2-4,6-8").unwrap().overlaps(), false);
        assert_eq!(AssignmentPair::from_str("5-7,7-9").unwrap().overlaps(), true);
        assert_eq!(AssignmentPair::from_str("2-8,3-7").unwrap().overlaps(), true);
        assert_eq!(AssignmentPair::from_str("6-6,4-6").unwrap().overlaps(), true);
        assert_eq!(AssignmentPair::from_str("2-6,4-8").unwrap().overlaps(), true);
    }

    #[test]
    fn part2() {
        assert_eq!(count_overlaps("./data/04.test").unwrap(), 4);
    }
    
}
