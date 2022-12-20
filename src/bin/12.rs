use std::collections::{HashMap, HashSet};
use anyhow::Result;

fn solve(hill: &Vec<Vec<u8>>, starting_pos: (usize, usize), ending_pos: (usize, usize)) -> usize {
    let mut dist = HashMap::new();
    let mut unvisited = HashSet::new();

    for (j, _) in hill.iter().enumerate() {
        for (i, _) in hill[j].iter().enumerate() {
            dist.insert((i, j), 9999);
            unvisited.insert((i, j));
        }
    }
    dist.insert(starting_pos, 0);

    while !unvisited.is_empty() {
        let (x, y) : (usize, usize) = *dist
            .iter()
            .filter(|item| unvisited.contains(item.0))
            .min_by(|a, b| a.1.cmp(&b.1))
            .unwrap()
            .0;
        unvisited.remove(&(x, y));

        let mut neighbours : Vec<(usize, usize)> = Vec::new();
        if x > 0 { neighbours.push((x - 1, y)) };
        if y > 0 { neighbours.push((x, y - 1)) };
        if x < hill[0].len() - 1 { neighbours.push((x + 1, y)) };
        if y < hill.len() - 1 { neighbours.push((x, y + 1)) };

        let cur_dist = dist[&(x, y)] + 1;
        for neigh in neighbours {
            let neigh_x = neigh.0;
            let neigh_y = neigh.1;
            if hill[neigh_y][neigh_x] <= hill[y][x] + 1 {
                if dist[&(neigh_x, neigh_y)] > cur_dist {
                    dist.insert((neigh_x, neigh_y), cur_dist);
                }
            }
        }
    }
    dist[&ending_pos]
}

fn main() -> Result<()> {
    let mut starting_pos = (0, 0);
    let mut ending_pos = (0, 0);
    let hill : Vec<Vec<u8>> = std::fs::read_to_string("./data/12.input")?
        .lines()
        .enumerate()
        .map(|(j, line)| line.parse::<String>().unwrap()
            .chars()
            .enumerate()
            .map(|(i, c)| match c {
                'S' => { starting_pos = (i, j); 0 }
                'E' => { ending_pos = (i, j); 26 }
                _ => c as u8 - b'a' + 1
            })
            .collect()
        )
        .collect();

    println!("Fewest steps part 1: {}", solve(&hill, starting_pos, ending_pos));

    let mut length_of_possible_trails = HashSet::new();
    for (j, _) in hill.iter().enumerate() {
        // Optimization: we can iterate only first column, since
        // 'a' chars placed elsewhere are completely surrounded by 'c' chars.
        //for (i, _) in hill[j].iter().enumerate() {
            if hill[j][0] <= 1 {
                starting_pos = (0, j);
                length_of_possible_trails.insert(solve(&hill, starting_pos, ending_pos));
            }
        //}
    }
    println!("Fewest steps part 2: {}", length_of_possible_trails.iter().min().unwrap());

    Ok(())
}