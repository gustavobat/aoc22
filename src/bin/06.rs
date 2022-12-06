use anyhow::Result;
use std::{collections::HashSet, ops::Add};

fn find_distinct_consecutive_chars(buffer: &String, n: usize) -> usize {
    buffer.chars()
        .collect::<Vec<char>>()
        .windows(n)
        .position( |w| { 
            let mut uniq = HashSet::new();
            w.into_iter().all(move |x| uniq.insert(x))
        })
        .expect("Could not find first marker!")
        .add(n)
}

fn main() -> Result<()> {
    let buffer = std::fs::read_to_string("./data/06.input")?;
    println!("Start-of-packet marker: {}", find_distinct_consecutive_chars(&buffer, 4));
    println!("Start-of-message marker: {}", find_distinct_consecutive_chars(&buffer, 14));

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn start_of_packet() {
        let buffer = std::fs::read_to_string("./data/06.test")
            .expect("Error reading input test file");
        assert_eq!(find_distinct_consecutive_chars(&buffer, 4), 7);
    }

    #[test]
    fn start_of_message() {
        let buffer = std::fs::read_to_string("./data/06.test")
            .expect("Error reading input test file");
        assert_eq!(find_distinct_consecutive_chars(&buffer, 14), 19);
    }
}
