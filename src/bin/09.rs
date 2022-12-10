use anyhow::Result;
use aoc22::read_one_per_line;
use std::{collections::HashSet, cmp::Ordering};

fn apply_movement(rope : &mut Vec<(i32, i32)>, direction: &str) {
    let dir_offset : (i32, i32) = match direction {
        "R" => Some((0, 1)),
        "L" => Some((0, -1)),
        "U" => Some((1, 0)),
        "D" => Some((-1, 0)),
        _ => None,
    }.expect("Invalid direction");

    // Move head
    rope[0].0 += dir_offset.0;
    rope[0].1 += dir_offset.1;

    for i in 1..rope.len() {
        // Calc distance to previous node
        let diff_x = rope[i - 1].0 - rope[i].0;
        let diff_y = rope[i - 1].1 - rope[i].1;
        let distance_to_prev = ((diff_x.pow(2) + diff_y.pow(2)) as f64).sqrt();
        if distance_to_prev> 1.5 { // Any value grater than sqrt(2.0)
            let x_offset = match diff_x.cmp(&0) {
                Ordering::Greater => 1,
                Ordering::Less => -1,
                Ordering::Equal => 0,
            };
            let y_offset = match diff_y.cmp(&0) {
                Ordering::Greater => 1,
                Ordering::Less => -1,
                Ordering::Equal => 0,
            };
            // Move node
            rope[i].0 += x_offset;
            rope[i].1 += y_offset;
        }
    }
}

fn calculate_visited_positions(movements: &Vec<String>, n_knots: usize) -> usize {
    let mut visited_positions = HashSet::<(i32, i32)>::new();

    let mut rope : Vec<(i32, i32)>= vec!((0,0); n_knots);

    visited_positions.insert(rope[1]);

    for line in movements {
        let (direction, n_steps_str) = line.split_once(" ").unwrap();
        let n_steps = n_steps_str.parse::<u32>().unwrap();
        for _ in 0..n_steps {
            apply_movement(&mut rope, direction);
            visited_positions.insert(*rope.last().unwrap());
        }
    }

    visited_positions.len()
}

fn main() -> Result<()> {

    let movements = read_one_per_line::<String>("./data/09.input")?;

    println!("Visited positions with two knots: {}", calculate_visited_positions(&movements, 2));
    println!("Visited positions with ten knots: {}", calculate_visited_positions(&movements, 10));

    Ok(())
}
