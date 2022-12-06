use anyhow::Result;
use std::collections::VecDeque;
use regex::Regex;

fn insert_crate(crates: &mut Vec<VecDeque<char>>, pos: usize, c: char) {
    if crates.len() <= pos + 1 {
        crates.resize(pos + 1, VecDeque::<char>::new());
    }
    crates[pos].push_front(c);
}

fn move_one_crate_at_a_time(crates: &mut Vec<VecDeque<char>>, from: usize, to: usize, n: usize) {
    for _ in 0..n {
        let c = crates[from].pop_back().unwrap();
        crates[to].push_back(c);
    }
}

fn move_multiples_crates_at_once(crates: &mut Vec<VecDeque<char>>, from: usize, to: usize, n: usize) {
    for i in 0..n {
        let c = crates[from][crates[from].len() - n + i];
        crates[to].push_back(c);
    }
    for _ in 0..n {
        crates[from].pop_back();
    }
}

fn get_initial_crate_configuration(input_path: &str) -> Result<Vec<VecDeque<char>>> {
    let crate_config_file_part : Vec<String> = std::fs::read_to_string(input_path)?
        .lines()
        .filter_map(|line| {
            if line.contains('[') {
                Some(line.to_string())
            } else {
                None
            }
        })
    .collect();

    let mut crates : Vec<VecDeque<char>> = Vec::new();
    for line in crate_config_file_part {
        for (i, c) in line.chars().enumerate() {
            if i % 4 == 1 && c.is_alphabetic() {
                insert_crate(&mut crates, i/4, c);
            } 
        }
    }

    Ok(crates)
}

fn rearrange_crates(mut crates: &mut Vec<VecDeque<char>>, input_path: &str, use_new_crane: bool) -> String {
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    std::fs::read_to_string(input_path)
        .expect("Could not open input path")
        .lines()
        .filter_map(|line| {
            if line.contains("move") { Some(line.to_string()) }
            else { None }
        })
        .for_each(|line| {
            let captures = re.captures(&line).expect(&line);
            if !use_new_crane {
                move_one_crate_at_a_time(&mut crates,
                                         captures[2].parse::<usize>().unwrap() - 1,
                                         captures[3].parse::<usize>().unwrap() - 1,
                                         captures[1].parse::<usize>().unwrap());
            } else {
                move_multiples_crates_at_once(&mut crates,
                                              captures[2].parse::<usize>().unwrap() - 1,
                                              captures[3].parse::<usize>().unwrap() - 1,
                                              captures[1].parse::<usize>().unwrap());
            }
        });

    let mut top_crates = String::new();
    for stack in crates {
        top_crates.push(stack.pop_back().unwrap());
    }
    top_crates
}

fn main() -> Result<()> {
    let input_path = "./data/05.input";
    let crates = get_initial_crate_configuration(input_path).unwrap();
    
    let mut use_new_crane = false;
    println!("Top crates with CrateMover9000: {}", 
             rearrange_crates(&mut crates.clone(), input_path, use_new_crane));
    use_new_crane = true;
    println!("Top crates with CrateMover9001: {}", 
             rearrange_crates(&mut crates.clone(), input_path, use_new_crane));

    Ok(())
}

