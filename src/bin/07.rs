use anyhow::Result;
use std::collections::HashMap;
use regex::Regex;

fn main() -> Result<()> {
    let console_lines = aoc22::read_one_per_line::<String>("./data/07.input")?;

    let file_regex = Regex::new(r"(\d+) .*").unwrap();
    let cd_regex = Regex::new(r"\$ cd (.*)").unwrap();

    let mut visited_dirs : Vec<String> = Vec::new();
    let mut size_per_dir: HashMap<String, usize> = HashMap::new();

    for line in console_lines {
        if line.as_str() == "$ cd .." {
            visited_dirs
                .pop()
                .expect("Can not pop any further");
        }
        else {
            if cd_regex.is_match(line.as_str()) {
                let dir = &cd_regex
                    .captures(line.as_str())
                    .expect("Unable to get regex capture group")[1];
                let dir_absolute_path : String;
                if dir == "/" {
                    dir_absolute_path = dir.to_string();
                }
                else {
                    dir_absolute_path = visited_dirs.last().unwrap().to_owned() + dir + "/";
                }

                visited_dirs.push(dir_absolute_path.to_string());
                size_per_dir.insert(dir_absolute_path.to_string(), 0);
            }
            else if file_regex.is_match(line.as_str()) {
                let file_size = file_regex
                    .captures(line.as_str())
                    .expect("Unable to get regex capture group")[1]
                    .parse::<usize>()
                    .expect("Could not parse capture group to 'usize' type");
                for dir in &visited_dirs {
                    *size_per_dir
                        .get_mut(dir)
                        .expect("Error reading dir size") += file_size;
                }
            }
        }
    }
    
    // Part 1
    let mut total : usize = 0;
    for (_, size) in size_per_dir.iter() {
        if *size <= 100_000 {
            total += *size;
        }
    }
    println!("Total size of <100_000 sized directories: {total}");

    // Part 2
    let root_size = *size_per_dir.get("/").unwrap();
    let space_to_be_freed = 30_000_000 - (70_000_000 - root_size);
    let mut min_dir_size_to_free = 70_000_000;

    for (_, size) in size_per_dir.iter() {
        if *size > space_to_be_freed && *size < min_dir_size_to_free {
            min_dir_size_to_free = *size;
        }
    }
    println!("Size of smallest dir that allows for update: {min_dir_size_to_free}");

    Ok(())
}
